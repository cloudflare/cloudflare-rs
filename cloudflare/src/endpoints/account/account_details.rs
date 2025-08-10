use crate::endpoints::account::Account;
use crate::framework::endpoint::{EndpointSpec, Method};

/// Account Details
/// <https://developers.cloudflare.com/api/resources/accounts/methods/get/>
#[derive(Debug)]
pub struct AccountDetails<'a> {
    pub account_identifier: &'a str,
}

impl<'a> EndpointSpec<Account> for AccountDetails<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}", self.account_identifier)
    }
}
