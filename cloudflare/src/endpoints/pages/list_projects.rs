use super::data_structures::PagesProject;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// List all Pages projects in an account
/// 
/// Fetches a list of all Pages projects owned by the account.
/// 
/// <https://api.cloudflare.com/#pages-project-get-projects>
#[derive(Debug)]
pub struct ListProjects<'a> {
    pub account_identifier: &'a str,
}

impl<'a> ListProjects<'a> {
    pub fn new(account_identifier: &'a str) -> Self {
        Self {
            account_identifier,
        }
    }
}

impl EndpointSpec for ListProjects<'_> {
    type JsonResponse = Vec<PagesProject>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("accounts/{}/pages/projects", self.account_identifier)
    }
}