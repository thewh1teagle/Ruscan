// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use window_shadows::set_shadow;

use pnet_datalink::NetworkInterface;
use serde::{Deserialize, Serialize};
mod utils;
mod network;
mod vendor;
use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, net::IpAddr, thread, time::Duration, process};
use env_logger;
use crate::{network::NetworkIterator, vendor::Vendor};
use log::{debug,error};

#[derive(Serialize, Deserialize, Debug)]
struct SelectInterface {
    name: String,
    index: u32
}

#[derive(Serialize, Deserialize, Debug)]
struct Host {
    host: String,
    mac: String,
    vendor: String,
    hostname: String
}

#[tauri::command]
fn get_interfaces() -> Vec<SelectInterface> {
    let mut raw_interfaces: Vec<NetworkInterface> = pnet_datalink::interfaces();
    raw_interfaces = raw_interfaces
        .iter()
        .filter(|i| !i.description.to_lowercase().contains("bluetooth"))
        .cloned()
        .collect();

    
    raw_interfaces.sort_by(|a, b| {
        let a_default = utils::is_default_interface(a);
        let b_default = utils::is_default_interface(b);
    
        // default interfaces come first
        b_default.cmp(&a_default)
    });
    let interfaces: Vec<SelectInterface> = raw_interfaces
        .iter()
        .map(|i| SelectInterface {index: i.index, name: i.description.clone()})
        .collect();


    
    return interfaces;
}

#[tauri::command]
async fn scan(interface: SelectInterface) -> Vec<Host> {
    let raw_interfaces: Vec<NetworkInterface> = pnet_datalink::interfaces();
    let selected = raw_interfaces.iter().find(|i| i.index == interface.index).unwrap();


    let ip_networks: Vec<&ipnetwork::IpNetwork> = selected.ips.iter().filter(|ip_network| ip_network.is_ipv4()).collect();
    let channel_config = pnet_datalink::Config {
        read_timeout: Some(Duration::from_millis(network::DATALINK_RCV_TIMEOUT)), 
        ..pnet_datalink::Config::default()
    };

    debug!("scanning with {interface:?}");
    let (mut tx, mut rx) = match pnet_datalink::channel(selected, channel_config) {
        Ok(pnet_datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            error!("Expected an Ethernet datalink channel");
            process::exit(1);
        },
        Err(error) => {
            error!("Datalink channel creation failed ({})", error);
            process::exit(1);
        }
    };

    // The 'timed_out' mutex is shared accross the main thread (which performs
    // ARP packet sending) and the response thread (which receives and stores
    // all ARP responses).
    let timed_out = Arc::new(AtomicBool::new(false));
    let cloned_timed_out = Arc::clone(&timed_out);
    let mut vendor_list = Vendor::new("assets/ieee-oui.csv");

    let arp_responses = thread::spawn(move || network::receive_arp_responses(&mut rx, cloned_timed_out, &mut vendor_list));

    let source_ip = network::find_source_ip(selected);
    let has_reached_timeout = Arc::new(AtomicBool::new(false));
    for _ in 0..2 {

        if has_reached_timeout.load(Ordering::Relaxed) {
            break;
        }

        let mut ip_addresses = NetworkIterator::new(&ip_networks);
        debug!("doing loop in {}", ip_addresses.len());
        for ip_address in ip_addresses {

            if has_reached_timeout.load(Ordering::Relaxed) {
                break;
            }

            if let IpAddr::V4(ipv4_address) = ip_address {
                debug!("sending arp to {ipv4_address:?}");
                network::send_arp_request(&mut tx, selected, source_ip, ipv4_address);
                // thread::sleep(Duration::from_millis(50));
            }
        }
    }
    debug!("done for loop");

    // Once the ARP packets are sent, the main thread will sleep for T seconds
    // (where T is the timeout option). After the sleep phase, the response
    // thread will receive a stop request through the 'timed_out' mutex.
    let mut sleep_ms_mount: u64 = 0;
    while !has_reached_timeout.load(Ordering::Relaxed) && sleep_ms_mount < 2000 {
        debug!("sleeping for 100ms");
        thread::sleep(Duration::from_millis(100));
        sleep_ms_mount += 100;
        debug!("sleep ms mount is {sleep_ms_mount}");
    }
    debug!("done while loop");
    timed_out.store(true, Ordering::Relaxed);

    let (_, target_details) = arp_responses.join().unwrap_or_else(|error| {
        error!("Failed to close receive thread ({:?})", error);
        process::exit(1);
    });
    debug!("response is {:?}", target_details);
    let mut found_hosts: Vec<Host> = target_details
        .iter()
        .map(|t| Host {host: t.ipv4.to_string(), hostname: t.hostname.clone().unwrap_or_default(), mac: t.mac.to_string(), vendor: t.vendor.clone().unwrap_or_default()})
        .collect();
    found_hosts.sort_by(|a, b| a.mac.cmp(&b.mac));
    return found_hosts;
}

fn main() {
    env_logger::init();
    let builder = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan, get_interfaces]);

    #[cfg(any(windows, target_os = "macos"))]
    let builder = builder
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
