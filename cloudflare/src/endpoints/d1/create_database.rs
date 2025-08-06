use super::D1Database;

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Create a new D1 database
/// 
/// Creates a new D1 database with the specified name.
/// Database names must be unique within the account.
/// 
/// <https://developers.cloudflare.com/api/resources/d1/methods/create/>
#[derive(Debug)]
pub struct CreateDatabase<'a> {
    pub account_identifier: &'a str,
    pub params: CreateDatabaseParams,
}

impl<'a> CreateDatabase<'a> {
    pub fn new(account_identifier: &'a str, params: CreateDatabaseParams) -> Self {
        Self {
            account_identifier,
            params,
        }
    }
}

impl EndpointSpec for CreateDatabase<'_> {
    type JsonResponse = D1Database;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("accounts/{}/d1/database", self.account_identifier)
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// D1 Primary Location Hint
/// 
/// Specify the region to create the D1 primary, if available.
/// If omitted, D1 will be created as close as possible to the current user.
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum D1PrimaryLocationHint {
    /// Western North America
    Wnam,
    /// Eastern North America  
    Enam,
    /// Western Europe
    Weur,
    /// Eastern Europe
    Eeur,
    /// Asia-Pacific
    Apac,
    /// Oceania
    Oc,
}

/// Parameters for creating a D1 database
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct CreateDatabaseParams {
    /// The name of the database to create
    pub name: String,
    /// Specify the region to create the D1 primary (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_location_hint: Option<D1PrimaryLocationHint>,
}

impl CreateDatabaseParams {
    pub fn new(name: String) -> Self {
        Self { 
            name,
            primary_location_hint: None,
        }
    }
    
    pub fn with_location_hint(name: String, location_hint: D1PrimaryLocationHint) -> Self {
        Self {
            name,
            primary_location_hint: Some(location_hint),
        }
    }
}