use crate::{json_content, Endpoint, JsonResponse, Method};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A Named Argo Tunnel
///
/// This is an Argo Tunnel that has been created. It can be used for routing and subsequent running.
/// https://api.cloudflare.com/#argo-tunnel-properties
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tunnel {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub connections: Vec<ActiveConnection>,
}

/// An active connection for a Named Argo Tunnel.
/// https://api.cloudflare.com/#argo-tunnel-properties
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ActiveConnection {
    pub colo_name: String,
    pub id: String,
    pub is_pending_reconnect: bool,
}

/// The result of a route request for a Named Argo Tunnel
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum RouteResult {
    Dns { cname: Change },
    LoadBalancer { load_balancer: Change, pool: Change },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Change {
    Unchanged,
    New,
    Updated,
}

/// Create a Named Argo Tunnel
///
/// This creates the Tunnel, which can then be routed and ran. Creating the Tunnel per se is only
/// a metadata operation (i.e. no Tunnel is running at this point).
/// https://api.cloudflare.com/#argo-tunnel-create-argo-tunnel
#[derive(Debug, Clone, PartialEq)]
pub struct Create<'a> {
    pub account_id: &'a str,
    pub params: CreateParams,
}
impl Endpoint for Create<'_> {
    type Body = CreateParams;
    type Query = ();
    type Response = JsonResponse<Tunnel>;

    const METHOD: Method = Method::Post;

    fn path(&self) -> Cow<str> {
        format!("accounts/{}/tunnels", self.account_id).into()
    }
    fn body(&self) -> &Self::Body {
        &self.params
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// Params for creating a Named Argo Tunnel
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateParams {
    /// The name for the Tunnel to be created. It must be unique within the account.
    pub name: String,
    /// The byte array (with 32 or more bytes) representing a secret for the tunnel. This is
    /// encoded into JSON as a base64 String. This secret is necessary to run the tunnel.
    #[serde(with = "crate::serializers::base64")]
    pub tunnel_secret: Vec<u8>,
}
json_content!(CreateParams);

/// List/search tunnels in an account.
/// https://api.cloudflare.com/#argo-tunnel-list-argo-tunnels
#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    pub account_id: &'a str,
    pub params: ListParams,
}

impl Endpoint for List<'_> {
    type Body = ListParams;
    type Query = ();
    type Response = JsonResponse<Vec<Tunnel>>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!("accounts/{}/tunnels", self.account_id).into()
    }
    fn body(&self) -> &Self::Body {
        &self.params
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// Params for filtering listed tunnels
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ListParams {
    pub name: Option<String>,
    pub uuid: Option<String>,
    pub is_deleted: Option<bool>,
    pub existed_at: Option<DateTime<Utc>>,
    pub name_prefix: Option<String>,
    pub was_inactive_at: Option<DateTime<Utc>>,
}

/// Delete a tunnel
/// https://api.cloudflare.com/#argo-tunnel-delete-argo-tunnel
#[derive(Debug, Clone, PartialEq)]
pub struct Delete<'a> {
    pub account_id: &'a str,
    pub tunnel_id: &'a str,
}

impl Endpoint for Delete<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<Vec<Tunnel>>;

    const METHOD: Method = Method::Delete;

    fn path(&self) -> Cow<str> {
        format!("accounts/{}/tunnels/{}", self.account_id, self.tunnel_id).into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}