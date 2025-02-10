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

pub enum RequestBody<'a> {
    Json(String),
    Raw(Vec<u8>),
    MultiPart(&'a dyn MultipartBody),
}

pub enum MultipartPart {
    Text(String),
    Bytes(Vec<u8>),
}

pub trait MultipartBody {
    /// Returns a list of parts to be included in a multipart request.
    /// Each part is a tuple of the part name and the part data.
    //
    // Client-agnostic implementation, because of the non-interoperability
    // between reqwest's blocking::multipart::Form/Part and async_impl::multipart::Form/Part.
    // Refactor this when reqwest has some sort of conversion between the two.
    fn parts(&self) -> Vec<(String, MultipartPart)>;
}

pub mod spec {
    use super::*;

    /// Represents a specification for an API call that can be built into an HTTP request and sent.
    /// New endpoints should implement this trait.
    ///
    /// If the request succeeds, the call will resolve to a `ResultType`.
    pub trait EndpointSpec {
        /// If the body of the response is raw bytes (Vec<u8>), set this to `true`. Defaults to `false`.
        const IS_RAW_BODY: bool = false;

        /// The JSON response type for this endpoint, if any.
        ///
        /// For endpoints that return either raw bytes or nothing, this should be `()`.
        type JsonResponse: ApiResult;
        /// The final response type for this endpoint.
        ///
        /// For endpoints that return raw bytes, this should be `Vec<u8>`.
        ///
        /// For endpoints that return JSON, this should be `ApiSuccess<Self::JsonResponse>`.
        type ResponseType;
        // The body type for this endpoint, if any.
        //
        // For endpoints that do not have a body, this should be `()`.
        // For endpoints that have a JSON body, this should be `String`.
        // type BodyType;

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
        fn body(&self) -> Option<RequestBody> {
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
        fn content_type(&self) -> Option<Cow<'static, str>> {
            match Self::body(self) {
                Some(RequestBody::Json(_)) => Some(Cow::Borrowed("application/json")),
                Some(RequestBody::Raw(_)) => Some(Cow::Borrowed("application/octet-stream")),
                Some(RequestBody::MultiPart(_)) => Some(Cow::Borrowed("multipart/form-data")),
                None => None,
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
