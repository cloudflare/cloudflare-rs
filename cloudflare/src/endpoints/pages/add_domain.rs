use super::data_structures::PagesDomain;

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Add a custom domain to a Pages project
/// 
/// Associates a custom domain with a Pages project.
/// 
/// <https://api.cloudflare.com/#pages-domains-add-domain>
#[derive(Debug)]
pub struct AddDomain<'a> {
    pub account_identifier: &'a str,
    pub project_name: &'a str,
    pub params: AddDomainParams,
}

impl<'a> AddDomain<'a> {
    pub fn new(
        account_identifier: &'a str,
        project_name: &'a str,
        params: AddDomainParams,
    ) -> Self {
        Self {
            account_identifier,
            project_name,
            params,
        }
    }
}

impl EndpointSpec for AddDomain<'_> {
    type JsonResponse = PagesDomain;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/pages/projects/{}/domains",
            self.account_identifier, self.project_name
        )
    }

    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Parameters for adding a domain to a Pages project
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct AddDomainParams {
    /// The custom domain name to add
    pub name: String,
}

impl AddDomainParams {
    /// Create parameters for adding a domain
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_domain_params() {
        let params = AddDomainParams::new("example.com".to_string());
        assert_eq!(params.name, "example.com");

        let json = serde_json::to_string(&params).unwrap();
        let expected = r#"{"name":"example.com"}"#;
        assert_eq!(json, expected);
    }
}