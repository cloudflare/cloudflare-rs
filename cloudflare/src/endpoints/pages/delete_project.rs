use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Delete a Pages project
/// 
/// Permanently deletes a Pages project by name. This action cannot be undone.
/// 
/// <https://api.cloudflare.com/#pages-project-delete-project>
#[derive(Debug)]
pub struct DeleteProject<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
}

impl<'a> DeleteProject<'a> {
    pub fn new(account_identifier: &'a str, project_name: &'a str) -> Self {
        Self {
            account_identifier,
            project_name,
        }
    }
}

impl EndpointSpec for DeleteProject<'_> {
    type JsonResponse = ();
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}",
            self.account_identifier, self.project_name
        )
    }
}