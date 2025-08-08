use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pages Build Configuration
/// 
/// Configuration settings for building a Pages project.
///
/// <https://api.cloudflare.com/#pages-project-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesBuildConfig {
    /// Enable build caching for the project (optional)
    pub build_caching: Option<bool>,
    /// Command used to build project (optional)
    pub build_command: Option<String>,
    /// Output directory of the build (optional)
    pub destination_dir: Option<String>,
    /// Directory to run the command (optional)
    pub root_dir: Option<String>,
    /// The classifying tag for analytics (optional)
    pub web_analytics_tag: Option<String>,
    /// Environment variables for the build process (optional)
    pub env_vars: Option<HashMap<String, String>>,
}

impl Default for PagesBuildConfig {
    fn default() -> Self {
        Self {
            build_caching: None,
            build_command: None,
            destination_dir: None,
            root_dir: None,
            web_analytics_tag: None,
            env_vars: None,
        }
    }
}

/// Pages Deployment Trigger Metadata
/// 
/// Additional information about what caused the deployment.
///
/// <https://api.cloudflare.com/#pages-deployment-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesDeploymentTriggerMetadata {
    /// Where the trigger happened (optional)
    pub branch: Option<String>,
    /// Hash of the deployment trigger commit (optional)
    pub commit_hash: Option<String>,
    /// Message of the deployment trigger commit (optional)
    pub commit_message: Option<String>,
}

/// Pages Deployment Trigger Type
/// 
/// What caused the deployment.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PagesDeploymentTriggerType {
    /// Triggered by a git push
    Push,
    /// Triggered manually/ad-hoc
    AdHoc,
}

/// Pages Deployment Trigger
/// 
/// Information about what caused the deployment.
///
/// <https://api.cloudflare.com/#pages-deployment-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesDeploymentTrigger {
    /// What caused the deployment
    #[serde(rename = "type")]
    pub trigger_type: PagesDeploymentTriggerType,
    /// Additional info about the trigger (optional)
    pub metadata: Option<PagesDeploymentTriggerMetadata>,
}

/// Pages Deployment Stage
/// 
/// Information about a deployment stage (build, deploy, etc.).
///
/// <https://api.cloudflare.com/#pages-deployment-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesDeploymentStage {
    /// Stage name
    pub name: String,
    /// Stage started timestamp (optional)
    pub started_on: Option<String>,
    /// Stage ended timestamp (optional)
    pub ended_on: Option<String>,
    /// Stage status
    pub status: String,
}

/// Pages Deployment
/// 
/// Represents a deployment of a Pages project.
///
/// <https://api.cloudflare.com/#pages-deployment-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesDeployment {
    /// Deployment identifier
    pub id: String,
    /// Short identifier for the deployment
    pub short_id: String,
    /// Project identifier
    pub project_id: String,
    /// Project name
    pub project_name: String,
    /// Environment the deployment is for (production or preview)
    pub environment: String,
    /// Deployment URL
    pub url: String,
    /// When the deployment was created
    pub created_on: String,
    /// When the deployment was last modified (optional)
    pub modified_on: Option<String>,
    /// List of alias URLs pointing to this deployment (optional)
    pub aliases: Option<Vec<String>>,
    /// Build configuration used for this deployment (optional)
    pub build_config: Option<PagesBuildConfig>,
    /// Source information (optional)
    pub source: Option<HashMap<String, serde_json::Value>>,
    /// Deployment trigger information (optional)
    pub deployment_trigger: Option<PagesDeploymentTrigger>,
    /// Deployment stages (optional)
    pub stages: Option<Vec<PagesDeploymentStage>>,
    /// Environment variables for the deployment (optional)
    pub env_vars: Option<HashMap<String, String>>,
    /// Latest deployment status
    pub latest_stage: Option<PagesDeploymentStage>,
}

impl ApiResult for PagesDeployment {}
impl ApiResult for Vec<PagesDeployment> {}

/// Pages Deployment Configuration
/// 
/// Configuration for deployments in different environments.
///
/// <https://api.cloudflare.com/#pages-project-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesDeploymentConfig {
    /// Build configuration
    pub build_config: Option<PagesBuildConfig>,
    /// Environment variables
    pub env_vars: Option<HashMap<String, String>>,
    /// KV namespace bindings (optional)
    pub kv_namespaces: Option<HashMap<String, String>>,
    /// Durable Object bindings (optional)
    pub durable_objects: Option<HashMap<String, String>>,
    /// D1 database bindings (optional)
    pub d1_databases: Option<HashMap<String, String>>,
    /// R2 bucket bindings (optional)
    pub r2_buckets: Option<HashMap<String, String>>,
    /// Service bindings (optional)
    pub services: Option<HashMap<String, String>>,
    /// Compatibility date (optional)
    pub compatibility_date: Option<String>,
    /// Compatibility flags (optional)
    pub compatibility_flags: Option<Vec<String>>,
}

/// Pages Deployment Configurations
/// 
/// Deployment configurations for production and preview environments.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesDeploymentConfigs {
    /// Production deployment configuration
    pub production: Option<PagesDeploymentConfig>,
    /// Preview deployment configuration
    pub preview: Option<PagesDeploymentConfig>,
}

/// Pages Source Configuration
/// 
/// Git repository configuration for a Pages project.
///
/// <https://api.cloudflare.com/#pages-project-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesSource {
    /// Repository type (e.g., "github")
    #[serde(rename = "type")]
    pub source_type: String,
    /// Configuration details for the source
    pub config: HashMap<String, serde_json::Value>,
}

/// Pages Project
/// 
/// Represents a Cloudflare Pages project.
///
/// <https://api.cloudflare.com/#pages-project-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesProject {
    /// Project identifier
    pub id: String,
    /// Project name
    pub name: String,
    /// Project subdomain (e.g., "projectname.pages.dev")
    pub subdomain: String,
    /// Production branch of the project (used to identify production deployments)
    pub production_branch: Option<String>,
    /// List of associated custom domains (optional)
    pub domains: Option<Vec<String>>,
    /// Source configuration (optional)
    pub source: Option<PagesSource>,
    /// Build configuration (optional)
    pub build_config: Option<PagesBuildConfig>,
    /// Deployment configurations for different environments (optional)
    pub deployment_configs: Option<PagesDeploymentConfigs>,
    /// Most recent deployment to the repo (optional)
    pub latest_deployment: Option<PagesDeployment>,
    /// Most recent deployment to production (optional)
    pub canonical_deployment: Option<PagesDeployment>,
    /// When the project was created
    pub created_on: String,
    /// When the project was last modified (optional)
    pub modified_on: Option<String>,
}

impl ApiResult for PagesProject {}
impl ApiResult for Vec<PagesProject> {}

/// Pages Domain
/// 
/// Represents a custom domain associated with a Pages project.
///
/// <https://api.cloudflare.com/#pages-domains-properties>
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesDomain {
    /// Domain identifier
    pub id: String,
    /// Domain name
    pub name: String,
    /// Domain status
    pub status: String,
    /// Verification token (optional)
    pub verification_token: Option<String>,
    /// When the domain was created
    pub created_on: String,
    /// When the domain was last modified (optional)
    pub modified_on: Option<String>,
}

impl ApiResult for PagesDomain {}
impl ApiResult for Vec<PagesDomain> {}

/// Pages Deployment Logs
/// 
/// Build and deployment logs for a Pages deployment.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesDeploymentLogs {
    /// Log lines
    pub data: Vec<HashMap<String, serde_json::Value>>,
    /// Total log count
    pub total: Option<u64>,
    /// Whether this includes all logs
    pub includes_container_logs: Option<bool>,
}

impl ApiResult for PagesDeploymentLogs {}

/// Pages Build Cache Purge Result
/// 
/// Result of purging the build cache for a Pages project.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PagesBuildCachePurgeResult {
    /// Whether the purge was successful
    pub purged: bool,
}

impl ApiResult for PagesBuildCachePurgeResult {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pages_project_deserialization() {
        let json = r#"
        {
            "id": "23cef5d4-8b9a-4b0a-9b1a-2b3c4d5e6f7g",
            "name": "my-project",
            "subdomain": "my-project.pages.dev",
            "domains": ["example.com"],
            "created_on": "2024-01-01T00:00:00.000Z",
            "latest_deployment": {
                "id": "deployment-123",
                "short_id": "abc123",
                "project_id": "23cef5d4-8b9a-4b0a-9b1a-2b3c4d5e6f7g",
                "project_name": "my-project",
                "environment": "production",
                "url": "https://my-project.pages.dev",
                "created_on": "2024-01-01T00:00:00.000Z"
            }
        }
        "#;

        let project: PagesProject = serde_json::from_str(json).unwrap();
        assert_eq!(project.id, "23cef5d4-8b9a-4b0a-9b1a-2b3c4d5e6f7g");
        assert_eq!(project.name, "my-project");
        assert_eq!(project.subdomain, "my-project.pages.dev");
        assert_eq!(project.domains, Some(vec!["example.com".to_string()]));
        assert!(project.latest_deployment.is_some());
    }

    #[test]
    fn test_pages_deployment_deserialization() {
        let json = r#"
        {
            "id": "deployment-123",
            "short_id": "abc123",
            "project_id": "project-456",
            "project_name": "my-project",
            "environment": "production",
            "url": "https://abc123.my-project.pages.dev",
            "created_on": "2024-01-01T00:00:00.000Z",
            "aliases": ["https://my-project.pages.dev"],
            "deployment_trigger": {
                "type": "push",
                "metadata": {
                    "branch": "main",
                    "commit_hash": "abc123def456",
                    "commit_message": "Update homepage"
                }
            }
        }
        "#;

        let deployment: PagesDeployment = serde_json::from_str(json).unwrap();
        assert_eq!(deployment.id, "deployment-123");
        assert_eq!(deployment.short_id, "abc123");
        assert_eq!(deployment.environment, "production");
        assert_eq!(deployment.aliases, Some(vec!["https://my-project.pages.dev".to_string()]));
        
        let trigger = deployment.deployment_trigger.unwrap();
        assert_eq!(trigger.trigger_type, PagesDeploymentTriggerType::Push);
        
        let metadata = trigger.metadata.unwrap();
        assert_eq!(metadata.branch, Some("main".to_string()));
        assert_eq!(metadata.commit_hash, Some("abc123def456".to_string()));
    }

    #[test]
    fn test_pages_build_config_default() {
        let config = PagesBuildConfig::default();
        assert_eq!(config.build_caching, None);
        assert_eq!(config.build_command, None);
        assert_eq!(config.destination_dir, None);
        assert_eq!(config.root_dir, None);
    }

    #[test]
    fn test_pages_domain_deserialization() {
        let json = r#"
        {
            "id": "domain-123",
            "name": "example.com",
            "status": "active",
            "verification_token": "token123",
            "created_on": "2024-01-01T00:00:00.000Z"
        }
        "#;

        let domain: PagesDomain = serde_json::from_str(json).unwrap();
        assert_eq!(domain.id, "domain-123");
        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.status, "active");
        assert_eq!(domain.verification_token, Some("token123".to_string()));
    }
}