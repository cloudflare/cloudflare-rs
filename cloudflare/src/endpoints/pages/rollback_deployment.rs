use super::data_structures::PagesDeployment;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Rollback to a previous Pages deployment
/// 
/// Rollbacks the production deployment to a previous successful deployment.
/// You can only rollback to successful builds on production.
/// 
/// <https://api.cloudflare.com/#pages-deployment-rollback-deployment>
#[derive(Debug)]
pub struct RollbackDeployment<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
    pub deployment_id: &'a str,
}

impl<'a> RollbackDeployment<'a> {
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

impl EndpointSpec for RollbackDeployment<'_> {
    type JsonResponse = PagesDeployment;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/deployments/{}/rollback",
            self.account_identifier, self.project_name, self.deployment_id
        )
    }
}