use super::data_structures::{PagesBuildConfig, PagesDeployment};

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;
use std::collections::HashMap;

/// Create a new deployment for a Pages project
/// 
/// Creates a new deployment from the latest commit in the production branch,
/// or from uploaded files for manual deployments.
/// 
/// <https://api.cloudflare.com/#pages-deployment-create-deployment>
#[derive(Debug)]
pub struct CreateDeployment<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
    pub params: CreateDeploymentParams,
}

impl<'a> CreateDeployment<'a> {
    pub fn new(
        account_identifier: &'a str,
        project_name: &'a str,
        params: CreateDeploymentParams,
    ) -> Self {
        Self {
            account_identifier,
            project_name,
            params,
        }
    }
}

impl EndpointSpec for CreateDeployment<'_> {
    type JsonResponse = PagesDeployment;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/deployments",
            self.account_identifier, self.project_name
        )
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Parameters for creating a deployment
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct CreateDeploymentParams {
    /// Build configuration for this deployment (optional)
    pub build_config: Option<PagesBuildConfig>,
    /// Environment variables for this deployment (optional)
    pub env_vars: Option<HashMap<String, String>>,
    /// Manifest of files to upload (for direct uploads, optional)
    pub manifest: Option<HashMap<String, String>>,
    /// Branch to deploy from (optional, defaults to production branch)
    pub branch: Option<String>,
}

impl CreateDeploymentParams {
    /// Create a deployment from the default production branch
    pub fn new() -> Self {
        Self {
            build_config: None,
            env_vars: None,
            manifest: None,
            branch: None,
        }
    }

    /// Create a deployment from a specific branch
    pub fn from_branch(branch: String) -> Self {
        Self {
            build_config: None,
            env_vars: None,
            manifest: None,
            branch: Some(branch),
        }
    }

    /// Create a deployment with direct file upload
    pub fn with_manifest(manifest: HashMap<String, String>) -> Self {
        Self {
            build_config: None,
            env_vars: None,
            manifest: Some(manifest),
            branch: None,
        }
    }

    /// Add build configuration
    pub fn with_build_config(mut self, build_config: PagesBuildConfig) -> Self {
        self.build_config = Some(build_config);
        self
    }

    /// Add environment variables
    pub fn with_env_vars(mut self, env_vars: HashMap<String, String>) -> Self {
        self.env_vars = Some(env_vars);
        self
    }

    /// Set the branch to deploy from
    pub fn with_branch(mut self, branch: String) -> Self {
        self.branch = Some(branch);
        self
    }
}

impl Default for CreateDeploymentParams {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_deployment_params_default() {
        let params = CreateDeploymentParams::new();
        assert_eq!(params.build_config, None);
        assert_eq!(params.env_vars, None);
        assert_eq!(params.manifest, None);
        assert_eq!(params.branch, None);

        let json = serde_json::to_string(&params).unwrap();
        let expected = "{}";
        assert_eq!(json, expected);
    }

    #[test]
    fn test_create_deployment_params_from_branch() {
        let params = CreateDeploymentParams::from_branch("development".to_string());
        assert_eq!(params.branch, Some("development".to_string()));

        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{"branch":"development"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_create_deployment_params_with_manifest() {
        let mut manifest = HashMap::new();
        manifest.insert("index.html".to_string(), "content-hash-123".to_string());
        manifest.insert("style.css".to_string(), "content-hash-456".to_string());

        let params = CreateDeploymentParams::with_manifest(manifest.clone());
        assert_eq!(params.manifest, Some(manifest));
    }

    #[test]
    fn test_create_deployment_params_with_env_vars() {
        let mut env_vars = HashMap::new();
        env_vars.insert("API_KEY".to_string(), "secret123".to_string());
        env_vars.insert("NODE_ENV".to_string(), "production".to_string());

        let params = CreateDeploymentParams::new().with_env_vars(env_vars.clone());
        assert_eq!(params.env_vars, Some(env_vars));
    }

    #[test]
    fn test_create_deployment_params_chaining() {
        let mut env_vars = HashMap::new();
        env_vars.insert("NODE_ENV".to_string(), "staging".to_string());

        let params = CreateDeploymentParams::new()
            .with_branch("staging".to_string())
            .with_env_vars(env_vars.clone());

        assert_eq!(params.branch, Some("staging".to_string()));
        assert_eq!(params.env_vars, Some(env_vars));
    }
}