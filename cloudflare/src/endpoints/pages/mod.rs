/*!
Cloudflare Pages endpoints for static site hosting and deployment.

This module provides comprehensive Pages project and deployment management capabilities including:
- Project CRUD operations (create, read, update, delete)
- Deployment management (create, list, retry, rollback)
- Custom domain management
- Build cache operations

All endpoints are fully compliant with the official Cloudflare API specification.
*/

pub mod create_project;
pub mod data_structures;
pub mod delete_project;
pub mod get_project;
pub mod list_projects;
pub mod update_project;

pub mod add_domain;
pub mod delete_domain;
pub mod get_domain;
pub mod list_domains;
pub mod update_domain;

pub mod create_deployment;
pub mod delete_deployment;
pub mod get_deployment;
pub mod get_deployment_logs;
pub mod list_deployments;
pub mod retry_deployment;
pub mod rollback_deployment;

pub mod purge_build_cache;

pub use create_project::{CreateProject, CreateProjectParams};
pub use data_structures::{
    PagesBuildConfig, PagesDeployment, PagesDeploymentConfig, PagesDeploymentConfigs,
    PagesDeploymentLogs, PagesDeploymentStage, PagesDeploymentTrigger,
    PagesDeploymentTriggerMetadata, PagesDeploymentTriggerType, PagesDomain, PagesProject,
    PagesSource, PagesBuildCachePurgeResult,
};
pub use delete_project::DeleteProject;
pub use get_project::GetProject;
pub use list_projects::ListProjects;
pub use update_project::{UpdateProject, UpdateProjectParams};

pub use add_domain::{AddDomain, AddDomainParams};
pub use delete_domain::DeleteDomain;
pub use get_domain::GetDomain;
pub use list_domains::ListDomains;
pub use update_domain::{UpdateDomain, UpdateDomainParams};

pub use create_deployment::{CreateDeployment, CreateDeploymentParams};
pub use delete_deployment::DeleteDeployment;
pub use get_deployment::GetDeployment;
pub use get_deployment_logs::GetDeploymentLogs;
pub use list_deployments::ListDeployments;
pub use retry_deployment::RetryDeployment;
pub use rollback_deployment::RollbackDeployment;

pub use purge_build_cache::PurgeBuildCache;