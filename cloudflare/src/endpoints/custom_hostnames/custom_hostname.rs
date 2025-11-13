use crate::framework::endpoint::{serialize_query, EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderDirection {
    Asc,
    Desc,
}

/// List Custom Hostnames
/// List, search, sort, and filter all of your custom hostnames.
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/list/>
#[derive(Debug)]
pub struct ListCustomHostnames<'a> {
    pub zone_id: &'a str,
    pub params: ListCustomHostnamesParams,
}

impl EndpointSpec for ListCustomHostnames<'_> {
    type JsonResponse = Vec<CustomHostname>;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("zones/{}/custom_hostnames", self.zone_id)
    }

    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(&self.params)
    }
}

/// Custom Hostname Details
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/get/>
#[derive(Debug)]
pub struct GetCustomHostname<'a> {
    pub zone_id: &'a str,
    pub identifier: &'a str,
}

impl EndpointSpec for GetCustomHostname<'_> {
    type JsonResponse = CustomHostname;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_id, self.identifier
        )
    }
}

/// Create Custom Hostname
/// Add a new custom hostname and request that an SSL certificate be issued for it.
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/create/>
#[derive(Debug)]
pub struct CreateCustomHostname<'a> {
    pub zone_id: &'a str,
    pub params: CreateCustomHostnameParams<'a>,
}

impl EndpointSpec for CreateCustomHostname<'_> {
    type JsonResponse = CustomHostname;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("zones/{}/custom_hostnames", self.zone_id)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Update Custom Hostname
/// Modify SSL configuration for a custom hostname.
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/edit/>
#[derive(Debug)]
pub struct UpdateCustomHostname<'a> {
    pub zone_id: &'a str,
    pub identifier: &'a str,
    pub params: UpdateCustomHostnameParams<'a>,
}

impl EndpointSpec for UpdateCustomHostname<'_> {
    type JsonResponse = CustomHostname;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_id, self.identifier
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Delete Custom Hostname
/// Delete Custom Hostname (and any issued SSL certificates)
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/methods/delete/>
#[derive(Debug)]
pub struct DeleteCustomHostname<'a> {
    pub zone_id: &'a str,
    pub identifier: &'a str,
}

impl EndpointSpec for DeleteCustomHostname<'_> {
    type JsonResponse = DeleteCustomHostnameResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "zones/{}/custom_hostnames/{}",
            self.zone_id, self.identifier
        )
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ListCustomHostnamesParams {
    /// Hostname ID to match against
    pub id: Option<String>,
    /// Fully qualified domain name to match against
    pub hostname: Option<String>,
    /// Page number of paginated results
    pub page: Option<u32>,
    /// Number of hostnames per page
    pub per_page: Option<u32>,
    /// Field to order hostnames by
    pub order: Option<ListCustomHostnamesOrder>,
    /// Direction to order hostnames
    pub direction: Option<OrderDirection>,
    /// Whether to filter hostnames based on if they have SSL enabled
    pub ssl: Option<u8>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ListCustomHostnamesOrder {
    Ssl,
    SslStatus,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct CreateCustomHostnameParams<'a> {
    /// The custom hostname that will point to your hostname via CNAME
    pub hostname: &'a str,
    /// SSL properties for the custom hostname
    pub ssl: SslParams<'a>,
    /// Unique key/value metadata for this hostname
    pub custom_metadata: Option<serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct UpdateCustomHostnameParams<'a> {
    /// SSL properties for the custom hostname
    pub ssl: Option<SslParams<'a>>,
    /// A hostname that will be sent to your custom origin server as SNI
    pub custom_origin_server: Option<&'a str>,
    /// A hostname that will be sent to your custom origin SNI
    pub custom_origin_sni: Option<&'a str>,
    /// Unique key/value metadata for this hostname
    pub custom_metadata: Option<serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, Default)]
pub struct SslParams<'a> {
    /// Domain control validation method
    pub method: Option<ValidationMethod>,
    /// Level of validation to be used for this hostname
    #[serde(rename = "type")]
    pub validation_type: Option<ValidationType>,
    /// The certificate authority that will issue the certificate
    pub certificate_authority: Option<CertificateAuthority>,
    /// Indicates whether the certificate covers a wildcard
    pub wildcard: Option<bool>,
    /// Custom certificate used for this hostname
    pub custom_certificate: Option<&'a str>,
    /// Custom CSR ID
    pub custom_csr_id: Option<&'a str>,
    /// Custom private key
    pub custom_key: Option<&'a str>,
    /// Bundle method
    pub bundle_method: Option<BundleMethod>,
    /// SSL certificate settings
    pub settings: Option<SslSettings>,
}

#[derive(Deserialize, Debug)]
pub struct DeleteCustomHostnameResponse {
    pub id: String,
}

/// A Custom Hostname
/// <https://developers.cloudflare.com/api/resources/custom_hostnames/>
#[derive(Deserialize, Debug)]
pub struct CustomHostname {
    /// Custom hostname identifier tag
    pub id: String,
    /// The custom hostname that will point to your hostname
    pub hostname: String,
    /// SSL properties for the custom hostname
    pub ssl: Ssl,
    /// Status of the hostname
    pub status: CustomHostnameStatus,
    /// These are errors that were encountered while trying to activate a hostname
    pub verification_errors: Option<Vec<String>>,
    /// This is a record which can be placed to activate a hostname
    pub ownership_verification: Option<OwnershipVerification>,
    /// This presents the token to be served by the given http url to activate a hostname
    pub ownership_verification_http: Option<OwnershipVerificationHttp>,
    /// A hostname that will be sent to your custom origin server
    pub custom_origin_server: Option<String>,
    /// A hostname that will be sent to your custom origin SNI for TLS handshake
    pub custom_origin_sni: Option<String>,
    /// Unique key/value metadata for this hostname
    pub custom_metadata: Option<serde_json::Value>,
    /// This is the time the hostname was created
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CustomHostnameStatus {
    Active,
    Pending,
    ActiveRedeploying,
    Moved,
    PendingDeletion,
    Deleted,
    PendingBlocked,
    PendingMigration,
    PendingProvisioned,
    TestPending,
    TestActive,
    TestActiveApex,
    TestBlocked,
    TestFailed,
    Provisioned,
    Blocked,
}

#[derive(Deserialize, Debug)]
pub struct Ssl {
    /// Custom hostname SSL identifier tag
    pub id: Option<String>,
    /// Status of the hostname's SSL certificates
    pub status: Option<SslStatus>,
    /// Domain control validation method
    pub method: Option<ValidationMethod>,
    /// Level of validation to be used for this hostname
    #[serde(rename = "type")]
    pub validation_type: Option<ValidationType>,
    /// The certificate authority that will issue the certificate
    pub certificate_authority: Option<CertificateAuthority>,
    /// Indicates whether the certificate covers a wildcard
    pub wildcard: Option<bool>,
    /// SSL specific errors
    pub validation_errors: Option<Vec<ValidationError>>,
    /// Validation records
    pub validation_records: Option<Vec<ValidationRecord>>,
    /// Custom certificate used for this hostname
    pub custom_certificate: Option<String>,
    /// Custom CSR ID
    pub custom_csr_id: Option<String>,
    /// Custom private key
    pub custom_key: Option<String>,
    /// Certificate pack identifier
    pub certificate_pack_id: Option<String>,
    /// Bundle method
    pub bundle_method: Option<BundleMethod>,
    /// SSL certificate settings
    pub settings: Option<SslSettings>,
    /// CNAME target for validation
    pub cname_target: Option<String>,
    /// CNAME for validation
    pub cname: Option<String>,
    /// HTTP URL for validation
    pub http_url: Option<String>,
    /// HTTP body for validation
    pub http_body: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SslStatus {
    Initializing,
    PendingValidation,
    Deleted,
    PendingIssuance,
    PendingDeployment,
    PendingDeletion,
    PendingExpiration,
    Expired,
    Active,
    InitializingTimedOut,
    ValidationTimedOut,
    IssuanceTimedOut,
    DeploymentTimedOut,
    DeletionTimedOut,
    PendingCleanup,
    StagingDeployment,
    StagingActive,
    DeactivatingTimedOut,
    InvalidRequestSsl,
    ProjectSsl,
    ProjectSslTimedOut,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ValidationMethod {
    Http,
    Txt,
    Email,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ValidationType {
    Dv,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CertificateAuthority {
    Digicert,
    Google,
    #[serde(rename = "lets_encrypt")]
    LetsEncrypt,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BundleMethod {
    Ubiquitous,
    Optimal,
    Force,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ValidationRecord {
    pub emails: Option<Vec<String>>,
    pub http_body: Option<String>,
    pub http_url: Option<String>,
    pub txt_name: Option<String>,
    pub txt_value: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ValidationError {
    pub message: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Switch {
    On,
    Off,
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct SslSettings {
    pub http2: Option<Switch>,
    pub min_tls_version: Option<String>,
    pub tls_1_3: Option<Switch>,
    pub ciphers: Option<Vec<String>>,
    pub early_hints: Option<Switch>,
}

#[derive(Deserialize, Debug)]
pub struct OwnershipVerification {
    #[serde(rename = "type")]
    pub verification_type: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OwnershipVerificationHttp {
    pub http_body: Option<String>,
    pub http_url: Option<String>,
}

impl ApiResult for CustomHostname {}
impl ApiResult for Vec<CustomHostname> {}
impl ApiResult for DeleteCustomHostnameResponse {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::endpoint::{EndpointSpec, Method};

    #[test]
    fn test_list_custom_hostnames_endpoint() {
        let endpoint = ListCustomHostnames {
            zone_id: "test_zone_123",
            params: ListCustomHostnamesParams {
                hostname: Some("example.com".to_string()),
                page: Some(1),
                per_page: Some(50),
                ..Default::default()
            },
        };

        assert_eq!(endpoint.method(), Method::GET);
        assert_eq!(endpoint.path(), "zones/test_zone_123/custom_hostnames");
        assert!(endpoint.query().is_some());
    }

    #[test]
    fn test_create_custom_hostname_endpoint() {
        let ssl_params = SslParams {
            method: Some(ValidationMethod::Http),
            validation_type: Some(ValidationType::Dv),
            certificate_authority: Some(CertificateAuthority::LetsEncrypt),
            wildcard: Some(false),
            custom_certificate: None,
            custom_csr_id: None,
            custom_key: None,
            bundle_method: Some(BundleMethod::Ubiquitous),
            settings: None,
        };

        let params = CreateCustomHostnameParams {
            hostname: "app.example.com",
            ssl: ssl_params,
            custom_metadata: None,
        };

        let endpoint = CreateCustomHostname {
            zone_id: "test_zone_123",
            params,
        };

        assert_eq!(endpoint.method(), Method::POST);
        assert_eq!(endpoint.path(), "zones/test_zone_123/custom_hostnames");
        assert!(endpoint.body().is_some());
    }

    #[test]
    fn test_update_custom_hostname_endpoint() {
        let ssl_params = SslParams {
            method: Some(ValidationMethod::Txt),
            validation_type: Some(ValidationType::Dv),
            certificate_authority: Some(CertificateAuthority::Google),
            wildcard: None,
            custom_certificate: None,
            custom_csr_id: None,
            custom_key: None,
            bundle_method: None,
            settings: None,
        };

        let params = UpdateCustomHostnameParams {
            ssl: Some(ssl_params),
            custom_origin_server: Some("new-origin.example.com"),
            custom_origin_sni: None,
            custom_metadata: None,
        };

        let endpoint = UpdateCustomHostname {
            zone_id: "test_zone_123",
            identifier: "hostname_456",
            params,
        };

        assert_eq!(endpoint.method(), Method::PATCH);
        assert_eq!(
            endpoint.path(),
            "zones/test_zone_123/custom_hostnames/hostname_456"
        );
        assert!(endpoint.body().is_some());
    }

    #[test]
    fn test_delete_custom_hostname_endpoint() {
        let endpoint = DeleteCustomHostname {
            zone_id: "test_zone_123",
            identifier: "hostname_789",
        };

        assert_eq!(endpoint.method(), Method::DELETE);
        assert_eq!(
            endpoint.path(),
            "zones/test_zone_123/custom_hostnames/hostname_789"
        );
    }

    #[test]
    fn test_ssl_params_serialization() {
        let ssl_params = SslParams {
            method: Some(ValidationMethod::Http),
            validation_type: Some(ValidationType::Dv),
            certificate_authority: Some(CertificateAuthority::Digicert),
            wildcard: Some(true),
            custom_certificate: Some("cert_data"),
            custom_csr_id: Some("csr_123"),
            custom_key: Some("key_data"),
            bundle_method: Some(BundleMethod::Optimal),
            settings: None,
        };

        let json = serde_json::to_string(&ssl_params).unwrap();
        assert!(json.contains("\"method\":\"http\""));
        assert!(json.contains("\"type\":\"dv\""));
        assert!(json.contains("\"certificate_authority\":\"digicert\""));
    }

    #[test]
    fn test_custom_hostname_status_deserialization() {
        let json = r#""active""#;
        let status: CustomHostnameStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, CustomHostnameStatus::Active));

        let json = r#""pending""#;
        let status: CustomHostnameStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, CustomHostnameStatus::Pending));
    }

    #[test]
    fn test_ssl_status_deserialization() {
        let json = r#""active""#;
        let status: SslStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, SslStatus::Active));

        let json = r#""pending_validation""#;
        let status: SslStatus = serde_json::from_str(json).unwrap();
        assert!(matches!(status, SslStatus::PendingValidation));
    }

    #[test]
    fn test_validation_method_serialization() {
        let method = ValidationMethod::Http;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, r#""http""#);

        let method = ValidationMethod::Txt;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, r#""txt""#);
    }

    #[test]
    fn test_certificate_authority_variants() {
        let ca = CertificateAuthority::LetsEncrypt;
        let json = serde_json::to_string(&ca).unwrap();
        assert_eq!(json, r#""lets_encrypt""#);

        let ca = CertificateAuthority::Google;
        let json = serde_json::to_string(&ca).unwrap();
        assert_eq!(json, r#""google""#);
    }

    #[test]
    fn test_list_params_default() {
        let params = ListCustomHostnamesParams::default();
        assert!(params.id.is_none());
        assert!(params.hostname.is_none());
        assert!(params.page.is_none());
    }

    #[test]
    fn test_custom_metadata_json() {
        let metadata = serde_json::json!({
            "app_id": "12345",
            "environment": "production"
        });

        let params = CreateCustomHostnameParams {
            hostname: "test.example.com",
            ssl: SslParams {
                method: Some(ValidationMethod::Http),
                validation_type: Some(ValidationType::Dv),
                certificate_authority: None,
                wildcard: None,
                custom_certificate: None,
                custom_csr_id: None,
                custom_key: None,
                bundle_method: None,
                settings: None,
            },
            custom_metadata: Some(metadata),
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("app_id"));
        assert!(json.contains("production"));
    }
}
