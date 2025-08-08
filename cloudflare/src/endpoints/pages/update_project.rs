use super::data_structures::{PagesBuildConfig, PagesDeploymentConfigs, PagesProject, PagesSource};

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Update an existing Pages project
/// 
/// Updates the configuration of an existing Pages project.
/// 
/// <https://api.cloudflare.com/#pages-project-update-project>
#[derive(Debug)]
pub struct UpdateProject<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
    pub params: UpdateProjectParams,
}

impl<'a> UpdateProject<'a> {
    pub fn new(
        account_identifier: &'a str,
        project_name: &'a str,
        params: UpdateProjectParams,
    ) -> Self {
        Self {
            account_identifier,
            project_name,
            params,
        }
    }
}

impl EndpointSpec for UpdateProject<'_> {
    type JsonResponse = PagesProject;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}",
            self.account_identifier, self.project_name
        )
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Parameters for updating a Pages project
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateProjectParams {
    /// Updated name for the project (optional)
    pub name: Option<String>,
    /// Updated subdomain for the project (optional)
    pub subdomain: Option<String>,
    /// Updated production branch for the project (optional)
    pub production_branch: Option<String>,
    /// Updated list of custom domains (optional)
    pub domains: Option<Vec<String>>,
    /// Updated source configuration (optional)
    pub source: Option<PagesSource>,
    /// Updated build configuration (optional)
    pub build_config: Option<PagesBuildConfig>,
    /// Updated deployment configurations (optional)
    pub deployment_configs: Option<PagesDeploymentConfigs>,
}

impl UpdateProjectParams {
    /// Create empty update parameters
    pub fn new() -> Self {
        Self {
            name: None,
            subdomain: None,
            production_branch: None,
            domains: None,
            source: None,
            build_config: None,
            deployment_configs: None,
        }
    }

    /// Update the project name
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Update the project subdomain
    pub fn with_subdomain(mut self, subdomain: String) -> Self {
        self.subdomain = Some(subdomain);
        self
    }

    /// Update the production branch
    pub fn with_production_branch(mut self, production_branch: String) -> Self {
        self.production_branch = Some(production_branch);
        self
    }

    /// Update the custom domains
    pub fn with_domains(mut self, domains: Vec<String>) -> Self {
        self.domains = Some(domains);
        self
    }

    /// Update the source configuration
    pub fn with_source(mut self, source: PagesSource) -> Self {
        self.source = Some(source);
        self
    }

    /// Update the build configuration
    pub fn with_build_config(mut self, build_config: PagesBuildConfig) -> Self {
        self.build_config = Some(build_config);
        self
    }

    /// Update the deployment configurations
    pub fn with_deployment_configs(mut self, deployment_configs: PagesDeploymentConfigs) -> Self {
        self.deployment_configs = Some(deployment_configs);
        self
    }
}

impl Default for UpdateProjectParams {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_project_params_empty() {
        let params = UpdateProjectParams::new();
        assert_eq!(params.name, None);
        assert_eq!(params.subdomain, None);
        assert_eq!(params.domains, None);

        let json = serde_json::to_string(&params).unwrap();
        let expected = "{}";
        assert_eq!(json, expected);
    }

    #[test]
    fn test_update_project_params_with_name() {
        let params = UpdateProjectParams::new().with_name("updated-project".to_string());
        assert_eq!(params.name, Some("updated-project".to_string()));

        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{"name":"updated-project"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_update_project_params_with_domains() {
        let params = UpdateProjectParams::new()
            .with_domains(vec!["newdomain.com".to_string(), "www.newdomain.com".to_string()]);

        assert_eq!(
            params.domains,
            Some(vec!["newdomain.com".to_string(), "www.newdomain.com".to_string()])
        );
    }

    #[test]
    fn test_update_project_params_chaining() {
        let params = UpdateProjectParams::new()
            .with_name("updated-project".to_string())
            .with_subdomain("new-subdomain".to_string())
            .with_domains(vec!["example.com".to_string()]);

        assert_eq!(params.name, Some("updated-project".to_string()));
        assert_eq!(params.subdomain, Some("new-subdomain".to_string()));
        assert_eq!(params.domains, Some(vec!["example.com".to_string()]));
    }
}