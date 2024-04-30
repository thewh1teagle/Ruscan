use network::{Host, SelectInterface};
use eyre::{ContextCompat, Result};


#[tauri::command]
pub fn get_interfaces() -> Result<Vec<SelectInterface>> {
    let interfaces = network::get_interfaces();
    
    Ok(interfaces)
}

#[tauri::command]
pub async fn scan(interface: SelectInterface) -> Result<Vec<Host>> {
    let interfaces = network::get_interfaces();
    let selected = interfaces.iter().find(|i| i.index == interface.index).context("not found")?;
    let found = network::scan(selected);
    found
}