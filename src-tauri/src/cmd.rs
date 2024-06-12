use blinkscan::{Host, Interface};
use eyre::Result;
use std::time::Duration;

#[tauri::command]
pub fn get_interfaces() -> Result<Vec<Interface>> {
    Ok(blinkscan::get_interfaces())
}
#[tauri::command]
pub async fn scan(interface: Interface) -> Result<Vec<Host>> {
    let network = blinkscan::create_network(&interface);
    let result: Vec<Host> = blinkscan::scan_network(network, Duration::from_secs(1)).collect();
    Ok(result)
}
