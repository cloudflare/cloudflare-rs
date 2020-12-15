use super::Account;

use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::OrderDirection;

/// List Accounts
/// List all accounts you have ownership or verified access to
/// https://api.cloudflare.com/#accounts-list-accounts
#[derive(Debug)]
pub struct ListAccounts {
    pub params: Option<ListAccountsParams>,
}

impl Endpoint<Vec<Account>, ListAccountsParams> for ListAccounts {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("accounts")
    }
    fn query(&self) -> Option<ListAccountsParams> {
        self.params.clone()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct ListAccountsParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub direction: Option<OrderDirection>,
}
