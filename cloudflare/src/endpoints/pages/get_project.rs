use super::data_structures::PagesProject;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Get a Pages project by name
/// 
/// Fetches details of a specific Pages project by name.
/// 
/// <https://api.cloudflare.com/#pages-project-get-project>
#[derive(Debug)]
pub struct GetProject<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
}

impl<'a> GetProject<'a> {
    pub fn new(account_identifier: &'a str, project_name: &'a str) -> Self {
        Self {
            account_identifier,
            project_name,
        }
    }
}

impl EndpointSpec for GetProject<'_> {
    type JsonResponse = PagesProject;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}",
            self.account_identifier, self.project_name
        )
    }
}