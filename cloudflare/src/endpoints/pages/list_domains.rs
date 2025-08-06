use super::data_structures::PagesDomain;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// List domains for a Pages project
/// 
/// Fetches a list of all custom domains associated with a Pages project.
/// 
/// <https://api.cloudflare.com/#pages-domains-get-domains>
#[derive(Debug)]
pub struct ListDomains<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
}

impl<'a> ListDomains<'a> {
    pub fn new(account_identifier: &'a str, project_name: &'a str) -> Self {
        Self {
            account_identifier,
            project_name,
        }
    }
}

impl EndpointSpec for ListDomains<'_> {
    type JsonResponse = Vec<PagesDomain>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/domains",
            self.account_identifier, self.project_name
        )
    }
}