///! An API client for the [Cloudflare API](https://api.cloudflare.com)
extern crate chrono;
extern crate reqwest;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate serde_qs;
extern crate url;

pub mod endpoints;
pub mod framework;
