use super::data_structures::PagesDomain;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Get a specific domain for a Pages project
/// 
/// Fetches details of a specific custom domain by name.
/// 
/// <https://api.cloudflare.com/#pages-domains-get-domain>
#[derive(Debug)]
pub struct GetDomain<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
    pub domain_name: &'a str,
}

impl<'a> GetDomain<'a> {
    pub fn new(
        account_identifier: &'a str,
        project_name: &'a str,
        domain_name: &'a str,
    ) -> Self {
        Self {
            account_identifier,
            project_name,
            domain_name,
        }
    }
}

impl EndpointSpec for GetDomain<'_> {
    type JsonResponse = PagesDomain;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/domains/{}",
            self.account_identifier, self.project_name, self.domain_name
        )
    }
}