use super::data_structures::PagesDeployment;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// List deployments for a Pages project
/// 
/// Fetches a list of all deployments for a specific Pages project.
/// 
/// <https://api.cloudflare.com/#pages-deployment-get-deployments>
#[derive(Debug)]
pub struct ListDeployments<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
}

impl<'a> ListDeployments<'a> {
    pub fn new(account_identifier: &'a str, project_name: &'a str) -> Self {
        Self {
            account_identifier,
            project_name,
        }
    }
}

impl EndpointSpec for ListDeployments<'_> {
    type JsonResponse = Vec<PagesDeployment>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/deployments",
            self.account_identifier, self.project_name
        )
    }
}