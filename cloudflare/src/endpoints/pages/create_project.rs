use super::data_structures::{PagesBuildConfig, PagesDeploymentConfigs, PagesProject, PagesSource};

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;
use std::collections::HashMap;

/// Create a new Pages project
/// 
/// Creates a new Pages project with the specified configuration.
/// Project names must be unique within the account.
/// 
/// <https://api.cloudflare.com/#pages-project-create-project>
#[derive(Debug)]
pub struct CreateProject<'a> {
    pub account_identifier: &'a str,
    pub params: CreateProjectParams,
}

impl<'a> CreateProject<'a> {
    pub fn new(account_identifier: &'a str, params: CreateProjectParams) -> Self {
        Self {
            account_identifier,
            params,
        }
    }
}

impl EndpointSpec for CreateProject<'_> {
    type JsonResponse = PagesProject;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("accounts/{}/pages/projects", self.account_identifier)
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Parameters for creating a Pages project
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct CreateProjectParams {
    /// The name of the project to create
    pub name: String,
    /// The subdomain for the project (optional, defaults to name if not provided)
    pub subdomain: Option<String>,
    /// Production branch for the project (optional, defaults to "main")
    pub production_branch: Option<String>,
    /// List of custom domains for the project (optional)
    pub domains: Option<Vec<String>>,
    /// Source configuration for Git integration (optional)
    pub source: Option<PagesSource>,
    /// Build configuration for the project (optional)
    pub build_config: Option<PagesBuildConfig>,
    /// Deployment configurations for different environments (optional)
    pub deployment_configs: Option<PagesDeploymentConfigs>,
}

impl CreateProjectParams {
    /// Create a new project with just a name
    pub fn new(name: String) -> Self {
        Self {
            name,
            subdomain: None,
            production_branch: None,
            domains: None,
            source: None,
            build_config: None,
            deployment_configs: None,
        }
    }

    /// Create a new project with name and subdomain
    pub fn with_subdomain(name: String, subdomain: String) -> Self {
        Self {
            name,
            subdomain: Some(subdomain),
            production_branch: None,
            domains: None,
            source: None,
            build_config: None,
            deployment_configs: None,
        }
    }

    /// Create a new project with Git source integration
    pub fn with_git_source(
        name: String,
        repo_type: String,
        repo_config: HashMap<String, serde_json::Value>,
    ) -> Self {
        let source = PagesSource {
            source_type: repo_type,
            config: repo_config,
        };

        Self {
            name,
            subdomain: None,
            production_branch: None,
            domains: None,
            source: Some(source),
            build_config: None,
            deployment_configs: None,
        }
    }

    /// Set production branch for the project
    pub fn with_production_branch(mut self, production_branch: String) -> Self {
        self.production_branch = Some(production_branch);
        self
    }

    /// Add custom domains to the project
    pub fn with_domains(mut self, domains: Vec<String>) -> Self {
        self.domains = Some(domains);
        self
    }

    /// Set build configuration
    pub fn with_build_config(mut self, build_config: PagesBuildConfig) -> Self {
        self.build_config = Some(build_config);
        self
    }

    /// Set deployment configurations
    pub fn with_deployment_configs(mut self, deployment_configs: PagesDeploymentConfigs) -> Self {
        self.deployment_configs = Some(deployment_configs);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_project_params_basic() {
        let params = CreateProjectParams::new("my-project".to_string());
        assert_eq!(params.name, "my-project");
        assert_eq!(params.subdomain, None);
        assert_eq!(params.domains, None);

        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{"name":"my-project"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_create_project_params_with_subdomain() {
        let params = CreateProjectParams::with_subdomain(
            "my-project".to_string(),
            "custom-subdomain".to_string(),
        );
        assert_eq!(params.name, "my-project");
        assert_eq!(params.subdomain, Some("custom-subdomain".to_string()));

        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{"name":"my-project","subdomain":"custom-subdomain"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_create_project_params_with_domains() {
        let params = CreateProjectParams::new("my-project".to_string())
            .with_domains(vec!["example.com".to_string(), "www.example.com".to_string()]);

        assert_eq!(params.domains, Some(vec!["example.com".to_string(), "www.example.com".to_string()]));
    }

    #[test]
    fn test_create_project_params_with_git_source() {
        let mut repo_config = HashMap::new();
        repo_config.insert("owner".to_string(), serde_json::Value::String("user".to_string()));
        repo_config.insert("repo_name".to_string(), serde_json::Value::String("my-repo".to_string()));

        let params = CreateProjectParams::with_git_source(
            "my-project".to_string(),
            "github".to_string(),
            repo_config.clone(),
        );

        assert_eq!(params.name, "my-project");
        assert!(params.source.is_some());
        
        let source = params.source.unwrap();
        assert_eq!(source.source_type, "github");
        assert_eq!(source.config, repo_config);
    }
}