use super::WorkersKvNamespace;

use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};

use crate::framework::response::ApiSuccess;
use serde::Serialize;

/// Creates a namespace under the given title.
///
/// A `400` is returned if the account already owns a namespace with this title.
/// A namespace must be explicitly deleted to be replaced.
///
/// <https://developers.cloudflare.com/api/resources/kv/subresources/namespaces/methods/create/>
#[derive(Debug)]
pub struct CreateNamespace<'a> {
    pub account_identifier: &'a str,
    pub params: CreateNamespaceParams,
}

impl EndpointSpec for CreateNamespace<'_> {
    type JsonResponse = WorkersKvNamespace;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("accounts/{}/storage/kv/namespaces", self.account_identifier)
    }
    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateNamespaceParams {
    pub title: String,
}
