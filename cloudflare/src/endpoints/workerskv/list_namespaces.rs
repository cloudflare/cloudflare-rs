use super::WorkersKvNamespace;

use crate::framework::endpoint::{serialize_query, EndpointSpec, Method};

use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// List Namespaces
/// Returns the namespaces owned by an account
/// <https://api.cloudflare.com/#workers-kv-namespace-list-namespaces>
#[derive(Debug)]
pub struct ListNamespaces<'a> {
    pub account_identifier: &'a str,
    pub params: ListNamespacesParams,
}

impl<'a> EndpointSpec for ListNamespaces<'a> {
    type JsonResponse = Vec<WorkersKvNamespace>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        format!("accounts/{}/storage/kv/namespaces", self.account_identifier)
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListNamespacesParams {
    pub direction: Option<Direction>,
    pub order: Option<Order>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Serialize, Clone, Debug)]
pub enum Direction {
    Asc,
    Desc,
}

#[derive(Serialize, Clone, Debug)]
pub enum Order {
    Id,
    Title,
}
