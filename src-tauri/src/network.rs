use std::process;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Instant;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::ErrorKind::TimedOut;

use dns_lookup::lookup_addr;
use ipnetwork::IpNetwork;
use pnet_datalink::{MacAddr, NetworkInterface, DataLinkSender, DataLinkReceiver};
use pnet::packet::{MutablePacket, Packet};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket, EtherTypes};
use pnet::packet::arp::{MutableArpPacket, ArpOperations, ArpHardwareTypes, ArpPacket};

use log::{error,debug};
use crate::vendor::Vendor;

pub const DATALINK_RCV_TIMEOUT: u64 = 500;

const ARP_PACKET_SIZE: usize = 28;

const ETHERNET_STD_PACKET_SIZE: usize = 42;

/**
 * Gives high-level details about the scan response. This may include Ethernet
 * details (packet count, size, ...) and other technical network aspects.
 */
pub struct ResponseSummary {
    pub packet_count: usize,
    pub arp_count: usize,
    pub duration_ms: u128
}

/**
 * A target detail represents a single host on the local network with an IPv4
 * address and a linked MAC address. Hostnames are optional since some hosts
 * does not respond to the resolve call (or the numeric mode may be enabled).
 */
#[derive(Debug)]
pub struct TargetDetails {
    pub ipv4: Ipv4Addr,
    pub mac: MacAddr,
    pub hostname: Option<String>,
    pub vendor: Option<String>
}

pub fn send_arp_request(tx: &mut Box<dyn DataLinkSender>, interface: &NetworkInterface, source_ip: Ipv4Addr, target_ip: Ipv4Addr) {

    let mut ethernet_buffer = vec![0u8; ETHERNET_STD_PACKET_SIZE];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap_or_else(|| {
        error!("Could not build Ethernet packet");
        process::exit(1);
    });

    let target_mac = MacAddr::broadcast();
    let source_mac = interface.mac.unwrap();
    ethernet_packet.set_destination(target_mac);
    ethernet_packet.set_source(source_mac);

    let selected_ethertype = EtherTypes::Arp;
    ethernet_packet.set_ethertype(selected_ethertype);

    let mut arp_buffer = [0u8; ARP_PACKET_SIZE];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap_or_else(|| {
        error!("Could not build ARP packet");
        process::exit(1);
    });

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(source_mac);
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(target_mac);
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet_mut());

    tx.send_to(ethernet_packet.to_immutable().packet(), Some(interface.clone()));
}

/**
 * A network iterator for iterating over multiple network ranges in with a
 * low-memory approach. This iterator was crafted to allow iteration over huge
 * network ranges (192.168.0.0/16) without consuming excessive memory.
 */
pub struct NetworkIterator {
    current_iterator: Option<ipnetwork::IpNetworkIterator>,
    networks: Vec<IpNetwork>,
    random_pool: Vec<IpAddr>
}

impl NetworkIterator {

    pub fn new(networks_ref: &[&IpNetwork]) -> NetworkIterator {

        // The IpNetwork struct implements the Clone trait, which means that a simple
        // dereference will clone the struct in the new vector
        let networks: Vec<IpNetwork> = networks_ref.iter().map(|network| *(*network)).collect();


        NetworkIterator {
            current_iterator: None,
            networks,
            random_pool: vec![]
        }
    }

    /**
     * The functions below are not public and only used by the Iterator trait
     * to help keep the next() code clean.
     */

    fn has_no_items_left(&self) -> bool {
        self.current_iterator.is_none() && self.networks.is_empty() && self.random_pool.is_empty()
    }

    fn select_new_iterator(&mut self) {

        self.current_iterator = Some(self.networks.remove(0).iter());
    }

    fn pop_next_iterator_address(&mut self) -> Option<IpAddr> {

        self.current_iterator.as_mut().map(|iterator| iterator.next()).unwrap_or(None)
    }

    pub fn len(&mut self) -> usize {
        return self.current_iterator.clone().into_iter().len();
    }

}

impl Iterator for NetworkIterator {

    type Item = IpAddr;

    fn next(&mut self) -> Option<Self::Item> {

        if self.has_no_items_left() {
            return None;
        }

        if self.current_iterator.is_none() && !self.networks.is_empty() {
            self.select_new_iterator();
        }


        let next_ip = self.pop_next_iterator_address();

        if next_ip.is_none() && !self.networks.is_empty() {
            self.select_new_iterator();
            return self.pop_next_iterator_address();
        }

        next_ip
    }
}

/**
 * Find the most adequate IPv4 address on a given network interface for sending
 * ARP requests. If the 'forced_source_ipv4' parameter is set, it will take
 * the priority over the network interface address.
 */
pub fn find_source_ip(network_interface: &NetworkInterface) -> Ipv4Addr {


    let potential_network = network_interface.ips.iter().find(|network| network.is_ipv4());
    match potential_network.map(|network| network.ip()) {
        Some(IpAddr::V4(ipv4_addr)) => ipv4_addr,
        _ => {
            error!("Expected IPv4 address on network interface");
            process::exit(1);
        }
    }
}

/**
 * Wait at least N seconds and receive ARP network responses. The main
 * downside of this function is the blocking nature of the datalink receiver:
 * when the N seconds are elapsed, the receiver loop will therefore only stop
 * on the next received frame. Therefore, the receiver should have been
 * configured to stop at certain intervals (500ms for example).
 */
pub fn receive_arp_responses(rx: &mut Box<dyn DataLinkReceiver>, timed_out: Arc<AtomicBool>, vendor_list: &mut Vendor) -> (ResponseSummary, Vec<TargetDetails>) {
    
    let mut discover_map: HashMap<Ipv4Addr, TargetDetails> = HashMap::new();
    let start_recording = Instant::now();

    let mut packet_count = 0;
    let mut arp_count = 0;

    loop {
        if timed_out.load(Ordering::Relaxed) {
            break;
        }

        let arp_buffer = match rx.next() {
            Ok(buffer) => buffer,
            Err(error) => {
                match error.kind() {
                    // The 'next' call will only block the thread for a given
                    // amount of microseconds. The goal is to avoid long blocks
                    // due to the lack of packets received.
                    TimedOut => continue,
                    _ => {
                        error!("Failed to receive ARP requests ({})", error);
                        process::exit(1);
                    }
                };
            }
        };
        packet_count += 1;
        
        let ethernet_packet = match EthernetPacket::new(arp_buffer) {
            Some(packet) => packet,
            None => continue
        };

        let is_arp_type = matches!(ethernet_packet.get_ethertype(), EtherTypes::Arp);
        if !is_arp_type {
            continue;
        }

        let arp_packet = ArpPacket::new(&arp_buffer[MutableEthernetPacket::minimum_packet_size()..]);
        arp_count += 1;

        // If we found an ARP packet, extract the details and add the essential
        // fields in the discover map. Please note that results are grouped by
        // IPv4 address - which means that a MAC change will appear as two
        // separete records in the result table.
        if let Some(arp) = arp_packet {

            let sender_ipv4 = arp.get_sender_proto_addr();
            let sender_mac = arp.get_sender_hw_addr();
    
            discover_map.insert(sender_ipv4, TargetDetails {
                ipv4: sender_ipv4,
                mac: sender_mac,
                hostname: None,
                vendor: None
            });
        }
    }

    // For each target found, enhance each item with additional results
    // results such as the hostname & MAC vendor.
    let target_details = discover_map.into_values().map(|mut target_detail| {

        target_detail.hostname = find_hostname(target_detail.ipv4);
        

        if vendor_list.has_vendor_db() {
            target_detail.vendor = vendor_list.search_by_mac(&target_detail.mac);
            debug!("found vendor is {:?}", target_detail.vendor);
        }

        target_detail

    }).collect();

    // The response summary can be used to display analytics related to the
    // performed ARP scans (packet counts, timings, ...)
    let response_summary = ResponseSummary {
        packet_count,
        arp_count,
        duration_ms: start_recording.elapsed().as_millis()
    };
    (response_summary, target_details)
}

/**
 * Find the local hostname linked to an IPv4 address. This will perform a
 * reverse DNS request in the local network to find the IPv4 hostname.
 */
fn find_hostname(ipv4: Ipv4Addr) -> Option<String> {

    let ip: IpAddr = ipv4.into();
    match lookup_addr(&ip) {
        Ok(hostname) => {

            // The 'lookup_addr' function returns an IP address if no hostname
            // was found. If this is the case, we prefer switching to None.
            if hostname.parse::<IpAddr>().is_ok() {
                return None; 
            }

            Some(hostname)
        },
        Err(_) => None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use ipnetwork::Ipv4Network;
    use std::env;

    #[test]
    fn should_resolve_public_ip() {

        // Sometimes, we do not have access to public networks in the test
        // environment and can pass the OFFLINE environment variable.
        if env::var("OFFLINE").is_ok() {
            assert_eq!(true, true);
        }
        else {
            let ipv4 = Ipv4Addr::new(1,1,1,1);
            assert_eq!(find_hostname(ipv4), Some("one.one.one.one".to_string()));
        }
    }

    #[test]
    fn should_resolve_localhost() {

        let ipv4 = Ipv4Addr::new(127,0,0,1);

        assert_eq!(find_hostname(ipv4), Some("localhost".to_string()));
    }

    #[test]
    fn should_not_resolve_unknown_ip() {

        let ipv4 = Ipv4Addr::new(10,254,254,254);

        assert_eq!(find_hostname(ipv4), None);
    }

    #[test]
    fn should_iterate_over_empty_networks() {

        let mut iterator = NetworkIterator::new(&vec![]);

        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn should_iterate_over_single_address() {

        let network_a = IpNetwork::V4(
            Ipv4Network::new(Ipv4Addr::new(192, 168, 1, 1), 32).unwrap()
        );
        let target_network: Vec<&IpNetwork> = vec![
            &network_a
        ];

        let mut iterator = NetworkIterator::new(&target_network);

        assert_eq!(iterator.next(), Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn should_iterate_over_multiple_address() {

        let network_a = IpNetwork::V4(
            Ipv4Network::new(Ipv4Addr::new(192, 168, 1, 1), 24).unwrap()
        );
        let target_network: Vec<&IpNetwork> = vec![
            &network_a
        ];

        let mut iterator = NetworkIterator::new(&target_network);

        assert_eq!(iterator.next(), Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 0))));
        assert_eq!(iterator.next(), Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
        assert_eq!(iterator.next(), Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2))));
    }

    #[test]
    fn should_iterate_over_multiple_networks() {

        let network_a = IpNetwork::V4(
            Ipv4Network::new(Ipv4Addr::new(192, 168, 1, 1), 32).unwrap()
        );
        let network_b = IpNetwork::V4(
            Ipv4Network::new(Ipv4Addr::new(10, 10, 20, 20), 32).unwrap()
        );
        let target_network: Vec<&IpNetwork> = vec![
            &network_a,
            &network_b
        ];

        let mut iterator = NetworkIterator::new(&target_network);

        assert_eq!(iterator.next(), Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
        assert_eq!(iterator.next(), Some(IpAddr::V4(Ipv4Addr::new(10, 10, 20, 20))));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn should_iterate_with_random() {

        let network_a = IpNetwork::V4(
            Ipv4Network::new(Ipv4Addr::new(192, 168, 1, 1), 32).unwrap()
        );
        let network_b = IpNetwork::V4(
            Ipv4Network::new(Ipv4Addr::new(10, 10, 20, 20), 32).unwrap()
        );
        let target_network: Vec<&IpNetwork> = vec![
            &network_a,
            &network_b
        ];

        let mut iterator = NetworkIterator::new(&target_network);

        assert_eq!(iterator.next().is_some(), true);
        assert_eq!(iterator.next().is_some(), true);
        assert_eq!(iterator.next(), None);
    }

}