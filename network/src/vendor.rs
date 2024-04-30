use std::io::Cursor;
use std::process;

use pnet_datalink::MacAddr;
use csv::{Position, Reader, ReaderBuilder};
use eyre::{Context, Result};
// The Vendor structure performs search operations on a vendor database to find
// which MAC address belongs to a specific vendor. All network vendors have a
// dedicated MAC address range that is registered by the IEEE and maintained in
// the OUI database. An OUI is a 24-bit globally unique assigned number
// referenced by various standards.
pub struct Vendor {
    reader: Option<Reader<Cursor<Vec<u8>>>>,
}

impl Vendor {

    // Create a new MAC vendor search instance based on the given datebase path
    // (absolute or relative). A failure will not throw an error, but leave the
    // vendor search instance without database reader.
    pub fn new() -> Self {
        
        let bytes = include_bytes!("../assets/ieee-oui.csv");
        let cursor = Cursor::new(bytes.to_vec());
        let reader = ReaderBuilder::new().has_headers(true).from_reader(cursor);

        Vendor { reader: Some(reader) }
    }

    pub fn has_vendor_db(&self) -> bool {
        self.reader.is_some()
    }

    // Find a vendor name based on a given MAC address. A vendor search
    // operation will perform a whole read on the database for now.
    pub fn search_by_mac(&mut self, mac_address: &MacAddr) -> Result<Option<String>> {

        match &mut self.reader {
            Some(reader) => {

                // The {:02X} syntax forces to pad all numbers with zero values.
                // This ensures that a MAC 002272... will not be printed as
                // 02272 and therefore fails the search process.
                let vendor_oui = format!("{:02X}{:02X}{:02X}", mac_address.0, mac_address.1, mac_address.2);

                // Since we share a common instance of the CSV reader, it must be reset
                // before each read (internal buffers will be cleared).
                reader.seek(Position::new()).context("Could not reset the CSV reader")?;

                for vendor_result in reader.records() {
            
                    let record = vendor_result.unwrap_or_else(|err| {
                        log::error!("Could not read CSV record ({})", err);
                        process::exit(1);
                    });
                    let potential_oui = record.get(1).unwrap_or("");
            
                    if vendor_oui.eq(potential_oui) {
                        return Ok(Some(record.get(2).unwrap_or("(no vendor)").to_string()))
                    }
                }

                Ok(None)            
            }
            None => Ok(None)
        }
    }
    
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_create_vendor_resolver() {
        
        let vendor = Vendor::new();

        assert_eq!(vendor.has_vendor_db(), true);
    }

    #[test]
    fn should_handle_unresolved_database() {
        
        let vendor = Vendor::new();

        assert_eq!(vendor.has_vendor_db(), false);
    }

    #[test]
    fn should_find_specific_mac_vendor() {
        
        let mut vendor = Vendor::new();
        let mac = MacAddr::new(0x40, 0x55, 0x82, 0xc3, 0xe5, 0x5b);

        assert_eq!(vendor.search_by_mac(&mac).unwrap(), Some("Nokia".to_string()));
    }

    #[test]
    fn should_find_first_mac_vendor() {
        
        let mut vendor = Vendor::new();
        let mac = MacAddr::new(0x00, 0x22, 0x72, 0xd7, 0xb5, 0x23);

        assert_eq!(vendor.search_by_mac(&mac).unwrap(), Some("American Micro-Fuel Device Corp.".to_string()));
    }

    #[test]
    fn should_find_last_mac_vendor() {
        
        let mut vendor = Vendor::new();
        let mac = MacAddr::new(0xcc, 0x9d, 0xa2, 0x14, 0x2e, 0x6f);

        assert_eq!(vendor.search_by_mac(&mac).unwrap(), Some("Eltex Enterprise Ltd.".to_string()));
    }

    #[test]
    fn should_handle_unknown_mac_vendor() {
        
        let mut vendor = Vendor::new();
        let mac = MacAddr::new(0xbb, 0xbb, 0xbb, 0xd2, 0xf5, 0xb6);

        assert_eq!(vendor.search_by_mac(&mac).unwrap(), None);
    }

    #[test]
    fn should_pad_correctly_with_zeroes() {
        
        let mut vendor = Vendor::new();
        let mac = MacAddr::new(0x01, 0x01, 0x01, 0x67, 0xb2, 0x1d);

        assert_eq!(vendor.search_by_mac(&mac).unwrap(), Some("SomeCorp".to_string()));
    }

}