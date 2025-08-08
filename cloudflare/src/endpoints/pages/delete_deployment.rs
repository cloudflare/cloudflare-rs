use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Delete a Pages deployment
/// 
/// Deletes a specific deployment by ID. This action cannot be undone.
/// 
/// <https://api.cloudflare.com/#pages-deployment-delete-deployment>
#[derive(Debug)]
pub struct DeleteDeployment<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
    pub deployment_id: &'a str,
}

impl<'a> DeleteDeployment<'a> {
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

impl EndpointSpec for DeleteDeployment<'_> {
    type JsonResponse = ();
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/deployments/{}",
            self.account_identifier, self.project_name, self.deployment_id
        )
    }
}