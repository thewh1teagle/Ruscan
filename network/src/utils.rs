use pnet_datalink::NetworkInterface;


pub fn is_default_interface(interface: &NetworkInterface) -> bool {
    
    if interface.mac.is_none() {
        return false;
    }

    if interface.ips.is_empty() || !interface.is_up() || interface.is_loopback() {
        return false;
    }

    let potential_ipv4 = interface.ips.iter().find(|ip| ip.is_ipv4());
    if potential_ipv4.is_none() {
        return false;
    }

    true
}