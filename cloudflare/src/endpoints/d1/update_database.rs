use super::D1Database;

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Update a D1 database
/// 
/// Updates configuration for an existing D1 database.
/// 
/// <https://developers.cloudflare.com/api/resources/d1/methods/update/>
#[derive(Debug)]
pub struct UpdateDatabase<'a> {
    pub account_identifier: &'a str,
    pub database_identifier: &'a str,
    pub params: UpdateDatabaseParams,
}

impl<'a> UpdateDatabase<'a> {
    pub fn new(
        account_identifier: &'a str,
        database_identifier: &'a str,
        params: UpdateDatabaseParams,
    ) -> Self {
        Self {
            account_identifier,
            database_identifier,
            params,
        }
    }
}

impl EndpointSpec for UpdateDatabase<'_> {
    type JsonResponse = D1Database;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/d1/database/{}",
            self.account_identifier, self.database_identifier
        )
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// D1 Read Replication Mode
/// 
/// Configuration for D1 read replication.
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum D1ReadReplicationMode {
    /// Create replicas automatically and place them around the world
    Auto,
    /// Disable database replicas (takes a few hours to delete all replicas)
    Disabled,
}

/// D1 Read Replication Configuration
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct D1ReadReplicationConfig {
    /// The read replication mode for the database
    pub mode: D1ReadReplicationMode,
}

/// Parameters for updating a D1 database
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateDatabaseParams {
    /// Configuration for D1 read replication
    pub read_replication: D1ReadReplicationConfig,
}

impl UpdateDatabaseParams {
    pub fn new(mode: D1ReadReplicationMode) -> Self {
        Self {
            read_replication: D1ReadReplicationConfig { mode },
        }
    }
}