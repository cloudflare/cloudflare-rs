use super::data_structures::PagesDomain;

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Update a custom domain for a Pages project
/// 
/// Updates the configuration of a custom domain associated with a Pages project.
/// 
/// <https://api.cloudflare.com/#pages-domains-patch-domain>
#[derive(Debug)]
pub struct UpdateDomain<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
    pub domain_name: &'a str,
    pub params: UpdateDomainParams,
}

impl<'a> UpdateDomain<'a> {
    pub fn new(
        account_identifier: &'a str,
        project_name: &'a str,
        domain_name: &'a str,
        params: UpdateDomainParams,
    ) -> Self {
        Self {
            account_identifier,
            project_name,
            domain_name,
            params,
        }
    }
}

impl EndpointSpec for UpdateDomain<'_> {
    type JsonResponse = PagesDomain;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/domains/{}",
            self.account_identifier, self.project_name, self.domain_name
        )
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Parameters for updating a domain
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateDomainParams {
    /// Updated domain name (optional)
    pub name: Option<String>,
}

impl UpdateDomainParams {
    /// Create empty update parameters
    pub fn new() -> Self {
        Self { name: None }
    }

    /// Update the domain name
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

impl Default for UpdateDomainParams {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_domain_params_empty() {
        let params = UpdateDomainParams::new();
        assert_eq!(params.name, None);

        let json = serde_json::to_string(&params).unwrap();
        let expected = "{}";
        assert_eq!(json, expected);
    }

    #[test]
    fn test_update_domain_params_with_name() {
        let params = UpdateDomainParams::new().with_name("newdomain.com".to_string());
        assert_eq!(params.name, Some("newdomain.com".to_string()));

        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{"name":"newdomain.com"}"#;
        assert_eq!(json, expected);
    }
}