use crate::framework::response::ApiResult;
use crate::framework::Environment;
use serde::Serialize;
use std::borrow::Cow;
use url::Url;

pub use http::Method;

// TODO: Unused feature?
#[cfg(feature = "endpoint-spec")]
pub use spec::EndpointSpec;
#[cfg(not(feature = "endpoint-spec"))]
pub(crate) use spec::EndpointSpec;

pub mod spec {
    use super::*;

    /// Represents a specification for an API call that can be built into an HTTP request and sent.
    /// New endpoints should implement this trait.
    ///
    /// If the request succeeds, the call will resolve to a `ResultType`.
    pub trait EndpointSpec {
        const IS_RAW_BODY: bool = false;

        type JsonResponse: ApiResult;
        type ResponseType;

        /// The HTTP Method used for this endpoint (e.g. GET, PATCH, DELETE)
        fn method(&self) -> Method;

        /// The relative URL path for this endpoint
        fn path(&self) -> String;

        /// The url-encoded query string associated with this endpoint. Defaults to `None`.
        ///
        /// Implementors should inline this.
        #[inline]
        fn query(&self) -> Option<String> {
            None
        }

        /// The HTTP body associated with this endpoint. If not implemented, defaults to `None`.
        ///
        /// Implementors should inline this.
        #[inline]
        fn body(&self) -> Option<String> {
            None
        }

        /// Builds and returns a formatted full URL, including query, for the endpoint.
        ///
        /// Implementors should generally not override this.
        fn url(&self, environment: &Environment) -> Url {
            let mut url = Url::from(environment).join(&self.path()).unwrap();
            url.set_query(self.query().as_deref());
            url
        }

        //noinspection RsConstantConditionIf
        /// If `body` is populated, indicates the body MIME type (defaults to JSON).
        ///
        /// Implementors generally do not need to override this.
        fn content_type(&self) -> Cow<'static, str> {
            // The condition is necessary, even if a warning is present.
            // The constant is overridden in some cases.
            if Self::IS_RAW_BODY {
                Cow::Borrowed("application/octet-stream")
            } else {
                Cow::Borrowed("application/json")
            }
        }
    }
}
// Auto-implement the public Endpoint trait for EndpointInternal implementors.
impl<T: ApiResult, U: EndpointSpec> Endpoint<T> for U {}

/// An API call that can be built into an HTTP request and sent.
///
/// If the request succeeds, the call will resolve to a `ResultType`.
pub trait Endpoint<ResultType: ApiResult>: EndpointSpec {}

/// A utility function for serializing parameters into a URL query string.
#[inline]
pub fn serialize_query<Q: Serialize>(q: &Q) -> Option<String> {
    serde_urlencoded::to_string(q).ok()
}
