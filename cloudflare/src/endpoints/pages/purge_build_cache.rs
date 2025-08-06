use super::data_structures::PagesBuildCachePurgeResult;

use crate::framework::endpoint::{EndpointSpec, Method};
use crate::framework::response::ApiSuccess;

/// Purge build cache for a Pages project
/// 
/// Purges all cached build artifacts for a Pages project, forcing the next
/// deployment to rebuild from scratch.
/// 
/// <https://api.cloudflare.com/#pages-purge-build-cache>
#[derive(Debug)]
pub struct PurgeBuildCache<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
}

impl<'a> PurgeBuildCache<'a> {
    pub fn new(account_identifier: &'a str, project_name: &'a str) -> Self {
        Self {
            account_identifier,
            project_name,
        }
    }
}

impl EndpointSpec for PurgeBuildCache<'_> {
    type JsonResponse = PagesBuildCachePurgeResult;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/purge_build_cache",
            self.account_identifier, self.project_name
        )
    }
}