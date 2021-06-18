use crate::{json_content, Endpoint, JsonResponse, Method};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Workers Route
///
/// Routes are basic patterns used to enable or disable workers that match requests.
/// https://api.cloudflare.com/#worker-routes-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Route {
    /// Route identifier tag.
    pub id: String,
    /// The basic pattern that should map to the script
    pub pattern: String,
    /// Name of the script to apply when the route is matched.
    /// The route is skipped when this is blank/missing.
    pub script: Option<String>,
}

// We could make `pattern` and `script` into `Option<String>` types
// but it feels wrong.
/// A variant of Route returned by the Create and Delete endpoints
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RouteId {
    /// Route identifier tag.
    pub id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    pub zone_id: &'a str,
}
impl Endpoint for List<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<Vec<Route>>;

    const METHOD: Method = Method::Get;

    fn path(&self) -> Cow<str> {
        format!("zones/{}/workers/routes", self.zone_id).into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Delete<'a> {
    pub zone_id: &'a str,
    pub route_id: &'a str,
}
impl Endpoint for Delete<'_> {
    type Body = ();
    type Query = ();
    type Response = JsonResponse<RouteId>;

    const METHOD: Method = Method::Delete;

    fn path(&self) -> Cow<str> {
        format!("zones/{}/workers/routes/{}", self.zone_id, self.route_id).into()
    }
    fn body(&self) -> &Self::Body {
        &()
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

/// Create a Route
///
/// Creates a route mapping the given pattern to the given script
/// https://api.cloudflare.com/#worker-routes-create-route
#[derive(Debug, Clone, PartialEq)]
pub struct Create<'a> {
    pub zone_id: &'a str,
    pub params: CreateParams,
}

impl Endpoint for Create<'_> {
    type Body = CreateParams;
    type Query = ();
    type Response = JsonResponse<RouteId>;

    const METHOD: Method = Method::Post;

    fn path(&self) -> Cow<str> {
        format!("zones/{}/workers/routes", self.zone_id).into()
    }
    fn body(&self) -> &Self::Body {
        &self.params
    }
    fn query(&self) -> &Self::Query {
        &()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreateParams {
    /// The zone name along with glob-style wildcards e.g. "example.net/*"
    pub pattern: String,
    /// Name of the script to apply when the route is matched. The route is
    /// skipped when this is blank/missing.
    pub script: Option<String>,
}
json_content!(CreateParams);
