use chrono::{offset::Utc, DateTime};
use uuid::Uuid;

use crate::framework::response::ApiResult;

pub mod create_tunnel;

/// A Named Argo Tunnel
/// This is an Argo Tunnel that has been created. It can be used for routing and subsequent running.
/// https://api.cloudflare.com/#argo-tunnel-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Tunnel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub connections: Vec<ActiveConnection>,
}

/// An active connection for a Named Argo Tunnel.
/// https://api.cloudflare.com/#argo-tunnel-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ActiveConnection {
    pub colo_name: String,
    pub id: Uuid,
    pub is_pending_reconnect: bool,
}

impl ApiResult for Tunnel {}
