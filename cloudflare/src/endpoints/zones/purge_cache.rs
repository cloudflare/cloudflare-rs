use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::framework::response::{ApiResult, ApiSuccess};
use serde::{Deserialize, Serialize};

/// Purge cached content for a zone, either everything or selectively.
/// <https://developers.cloudflare.com/api/resources/cache/methods/purge/>
#[derive(Debug)]
pub struct PurgeCache<'a> {
    pub zone_identifier: &'a str,
    pub params: PurgeCacheParams,
}

impl EndpointSpec for PurgeCache<'_> {
    type JsonResponse = PurgeCacheResponse;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("zones/{}/purge_cache", self.zone_identifier)
    }
    #[inline]
    fn body(&self) -> Option<RequestBody<'_>> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// What to purge. Tags, hosts and prefixes are not available on every plan.
#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum PurgeCacheParams {
    Everything {
        purge_everything: bool,
    },
    Selective {
        /// Exact URLs to purge
        #[serde(skip_serializing_if = "Vec::is_empty")]
        files: Vec<String>,
        /// Cache tags to purge
        #[serde(skip_serializing_if = "Vec::is_empty")]
        tags: Vec<String>,
        /// Hostnames to purge
        #[serde(skip_serializing_if = "Vec::is_empty")]
        hosts: Vec<String>,
        /// Host/path prefixes to purge
        #[serde(skip_serializing_if = "Vec::is_empty")]
        prefixes: Vec<String>,
    },
}

impl PurgeCacheParams {
    pub fn everything() -> Self {
        PurgeCacheParams::Everything {
            purge_everything: true,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PurgeCacheResponse {
    /// Identifier of the purge request
    pub id: String,
}

impl ApiResult for PurgeCacheResponse {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_purge_everything() {
        assert_eq!(
            serde_json::to_value(PurgeCacheParams::everything()).unwrap(),
            json!({ "purge_everything": true })
        );
    }

    #[test]
    fn selective_purge_omits_empty_lists() {
        let params = PurgeCacheParams::Selective {
            files: vec![],
            tags: vec![],
            hosts: vec!["www.example.com".into()],
            prefixes: vec![],
        };
        assert_eq!(
            serde_json::to_value(params).unwrap(),
            json!({ "hosts": ["www.example.com"] })
        );
    }
}
