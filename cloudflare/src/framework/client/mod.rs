use std::net::IpAddr;
use std::time::Duration;

pub mod async_api;
// There is no blocking support for wasm.
#[cfg(all(feature = "blocking", not(target_arch = "wasm32")))]
pub mod blocking_api;

/// Configuration for the API client. Allows users to customize its behaviour.
pub struct ClientConfig {
    /// The maximum time limit for an API request. If a request takes longer than this, it will be
    /// cancelled.
    /// Note: this configuration has no effect when the target is wasm32.
    pub http_timeout: Duration,
    /// A default set of HTTP headers which will be sent with each API request.
    pub default_headers: http::HeaderMap,
    /// A specific IP to use when establishing a connection
    /// Note: this configuration has no effect when the target is wasm32.
    pub resolve_ip: Option<IpAddr>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            http_timeout: Duration::from_secs(30),
            default_headers: http::HeaderMap::default(),
            resolve_ip: None,
        }
    }
}
