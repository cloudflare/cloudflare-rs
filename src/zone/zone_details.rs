use crate::endpoint::{Endpoint, Method};
use crate::zone::Zone;

/// Zone Details
/// https://api.cloudflare.com/#zone-zone-details
pub struct ZoneDetails<'a> {
    pub identifier: &'a str,
}
impl<'a> Endpoint<Zone> for ZoneDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("zones/{}", self.identifier)
    }
}
