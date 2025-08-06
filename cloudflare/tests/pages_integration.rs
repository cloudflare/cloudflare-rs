use cloudflare::endpoints::pages::*;
use std::collections::HashMap;

/// Test that Pages endpoints can be constructed successfully
#[test]
fn test_pages_endpoint_construction() {
    let account_id = "test-account";
    let project_name = "test-project";
    let deployment_id = "test-deployment";
    let domain_name = "example.com";

    // Test that all endpoints can be constructed without panicking
    let _list_projects = ListProjects::new(account_id);
    let _get_project = GetProject::new(account_id, project_name);
    
    let create_params = CreateProjectParams::new("my-project".to_string());
    let _create_project = CreateProject::new(account_id, create_params);
    
    let update_params = UpdateProjectParams::new();
    let _update_project = UpdateProject::new(account_id, project_name, update_params);
    
    let _delete_project = DeleteProject::new(account_id, project_name);

    // Test deployment endpoints
    let _list_deployments = ListDeployments::new(account_id, project_name);
    let _get_deployment = GetDeployment::new(account_id, project_name, deployment_id);
    
    let create_deployment_params = CreateDeploymentParams::new();
    let _create_deployment = CreateDeployment::new(account_id, project_name, create_deployment_params);
    
    let _retry_deployment = RetryDeployment::new(account_id, project_name, deployment_id);
    let _rollback_deployment = RollbackDeployment::new(account_id, project_name, deployment_id);

    // Test domain endpoints
    let _list_domains = ListDomains::new(account_id, project_name);
    
    let add_domain_params = AddDomainParams::new(domain_name.to_string());
    let _add_domain = AddDomain::new(account_id, project_name, add_domain_params);
    
    let _delete_domain = DeleteDomain::new(account_id, project_name, domain_name);
    let _get_domain = GetDomain::new(account_id, project_name, domain_name);

    // Test utility endpoints
    let _purge_build_cache = PurgeBuildCache::new(account_id, project_name);
    let _get_deployment_logs = GetDeploymentLogs::new(account_id, project_name, deployment_id);
}

#[test]
fn test_pages_parameter_builders() {
    // Test CreateProjectParams builders
    let basic_params = CreateProjectParams::new("my-project".to_string());
    assert_eq!(basic_params.name, "my-project");
    assert_eq!(basic_params.subdomain, None);

    let with_subdomain = CreateProjectParams::with_subdomain(
        "my-project".to_string(),
        "custom-subdomain".to_string(),
    );
    assert_eq!(with_subdomain.subdomain, Some("custom-subdomain".to_string()));

    let with_domains = basic_params.clone()
        .with_domains(vec!["example.com".to_string()]);
    assert_eq!(with_domains.domains, Some(vec!["example.com".to_string()]));

    // Test Git source creation
    let mut repo_config = HashMap::new();
    repo_config.insert("owner".to_string(), serde_json::Value::String("user".to_string()));
    repo_config.insert("repo_name".to_string(), serde_json::Value::String("repo".to_string()));

    let with_git = CreateProjectParams::with_git_source(
        "my-project".to_string(),
        "github".to_string(),
        repo_config.clone(),
    );
    assert!(with_git.source.is_some());
    assert_eq!(with_git.source.unwrap().source_type, "github");

    // Test UpdateProjectParams builders
    let update_params = UpdateProjectParams::new()
        .with_name("updated-name".to_string())
        .with_subdomain("new-subdomain".to_string());
    
    assert_eq!(update_params.name, Some("updated-name".to_string()));
    assert_eq!(update_params.subdomain, Some("new-subdomain".to_string()));

    // Test CreateDeploymentParams builders
    let deployment_params = CreateDeploymentParams::from_branch("staging".to_string());
    assert_eq!(deployment_params.branch, Some("staging".to_string()));

    let mut env_vars = HashMap::new();
    env_vars.insert("NODE_ENV".to_string(), "production".to_string());

    let with_env_vars = CreateDeploymentParams::new().with_env_vars(env_vars.clone());
    assert_eq!(with_env_vars.env_vars, Some(env_vars));

    // Test production branch functionality
    let with_prod_branch = CreateProjectParams::new("my-project".to_string())
        .with_production_branch("production".to_string());
    assert_eq!(with_prod_branch.production_branch, Some("production".to_string()));

    let update_prod_branch = UpdateProjectParams::new()
        .with_production_branch("develop".to_string());
    assert_eq!(update_prod_branch.production_branch, Some("develop".to_string()));
}