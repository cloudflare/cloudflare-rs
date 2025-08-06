use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Delete a custom domain from a Pages project
/// 
/// Removes a custom domain association from a Pages project.
/// 
/// <https://api.cloudflare.com/#pages-domains-delete-domain>
#[derive(Debug)]
pub struct DeleteDomain<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
    pub domain_name: &'a str,
}

impl<'a> DeleteDomain<'a> {
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

impl EndpointSpec for DeleteDomain<'_> {
    type JsonResponse = ();
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/domains/{}",
            self.account_identifier, self.project_name, self.domain_name
        )
    }
}