use super::data_structures::PagesDeployment;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Retry a failed Pages deployment
/// 
/// Retries a previous deployment that failed during the build or deploy process.
/// 
/// <https://api.cloudflare.com/#pages-deployment-retry-deployment>
#[derive(Debug)]
pub struct RetryDeployment<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
    pub deployment_id: &'a str,
}

impl<'a> RetryDeployment<'a> {
    pub fn new(
        account_identifier: &'a str,
        project_name: &'a str,
        deployment_id: &'a str,
    ) -> Self {
        Self {
            account_identifier,
            project_name,
            deployment_id,
        }
    }
}

impl EndpointSpec for RetryDeployment<'_> {
    type JsonResponse = PagesDeployment;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/deployments/{}/retry",
            self.account_identifier, self.project_name, self.deployment_id
        )
    }
}