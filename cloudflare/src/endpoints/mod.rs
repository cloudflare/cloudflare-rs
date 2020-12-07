/*!
Implementations of the Endpoint trait for individual Cloudflare API endpoints, e.g. DNS or Workers.
If you want to add a new Cloudflare API to this crate, simply add a new submodule of this `endpoints`
module.
 */
pub mod account;
pub mod argo_tunnel;
pub mod dns;
pub mod load_balancing;
pub mod plan;
pub mod user;
pub mod workers;
pub mod workerskv;
pub mod zone;
