use super::data_structures::PagesDeploymentLogs;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Get deployment logs for a Pages deployment
/// 
/// Fetches build and deployment logs for a specific deployment.
/// 
/// <https://api.cloudflare.com/#pages-deployment-get-deployment-logs>
#[derive(Debug)]
pub struct GetDeploymentLogs<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
    pub deployment_id: &'a str,
}

impl<'a> GetDeploymentLogs<'a> {
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

impl EndpointSpec for GetDeploymentLogs<'_> {
    type JsonResponse = PagesDeploymentLogs;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/deployments/{}/history/logs",
            self.account_identifier, self.project_name, self.deployment_id
        )
    }
}