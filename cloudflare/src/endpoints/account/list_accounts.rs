use super::Account;

use crate::framework::endpoint::{serialize_query, EndpointSpec, Method};
use crate::framework::OrderDirection;

use serde::Serialize;

/// List Accounts
/// List all accounts you have ownership or verified access to
/// <https://api.cloudflare.com/#accounts-list-accounts>
#[derive(Debug)]
pub struct ListAccounts {
    pub params: Option<ListAccountsParams>,
}

impl EndpointSpec<Vec<Account>> for ListAccounts {
    fn method(&self) -> Method {
        Method::GET
    }
    fn path(&self) -> String {
        "accounts".to_string()
    }
    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListAccountsParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub direction: Option<OrderDirection>,
}
