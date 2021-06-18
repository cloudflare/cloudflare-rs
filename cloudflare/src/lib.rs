//! An easy to use request abstraction for the [Cloudflare API](https://api.cloudflare.com)

pub mod api;
#[cfg(feature = "request")]
pub mod request;
mod serializers;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::borrow::Cow;

#[cfg(feature = "request")]
pub use request::{Request, RequestBuilder};

pub use http::{header::HeaderValue, status::StatusCode};

/// An HTTP API method
pub enum Method {
    /// The 'GET' method
    Get,
    /// The 'POST' method
    Post,
    /// The 'PUT' method
    Put,
    /// The 'DELETE' method
    Delete,
    /// The 'PATCH' method
    Patch,
}

/// A Cloudflare API endpoint call
pub trait Endpoint: Sized {
    /// The result of this endpoint call
    type Response;
    /// The body of this endpoint call
    type Body;
    /// The Query of this endpoint call
    type Query;

    /// The method of this endpoint
    const METHOD: Method;

    /// Gets the relative URL for this endpoint as a possibly owned string.
    /// This includes any possible query parameters.
    fn path(&self) -> Cow<str>;

    /// The query of this call
    fn query(&self) -> &Self::Query;

    /// Gets the body for this call
    fn body(&self) -> &Self::Body;
}

impl<T: Endpoint> Endpoint for &T {
    type Response = T::Response;
    type Body = T::Body;
    type Query = T::Query;

    const METHOD: Method = T::METHOD;

    fn path(&self) -> Cow<str> {
        (&**self).path()
    }
    fn query(&self) -> &Self::Query {
        (&**self).query()
    }
    fn body(&self) -> &Self::Body {
        (&**self).body()
    }
}

impl<T: Endpoint> Endpoint for &mut T {
    type Response = T::Response;
    type Body = T::Body;
    type Query = T::Query;

    const METHOD: Method = T::METHOD;

    fn path(&self) -> Cow<str> {
        (&**self).path()
    }
    fn query(&self) -> &Self::Query {
        (&**self).query()
    }
    fn body(&self) -> &Self::Body {
        (&**self).body()
    }
}

/// Represents content to be used as the body of an endpoint request.
pub trait Content {
    /// Serializes the content to construct the body of the request.
    /// Discarding by dropping or forgetting the serializer assumes an empty body.
    fn serialize<C: ContentSerializer>(&self, serializer: C) -> Result<C::Ok, C::Error>;
}

/// Implements the Content trait for json content structures.
#[macro_export]
macro_rules! json_content {
    ($ty:ty) => {
        impl $crate::Content for $ty {
            fn serialize<C: $crate::ContentSerializer>(
                &self,
                serializer: C,
            ) -> Result<C::Ok, C::Error> {
                serializer.json(self)
            }
        }
    };
}

/// A type used to serialize body content to an HTTP request.
pub trait ContentSerializer {
    /// The result type
    type Ok;
    /// The error type
    type Error;
    /// The type used for multipart serialization
    type MultipartSerializer: MultipartSerializer<Ok = Self::Ok, Error = Self::Error>;

    /// No content.
    fn empty(self) -> Result<Self::Ok, Self::Error>;

    /// Serializes a json body (content type 'application/json')
    fn json<T: Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error>;

    /// Returns a serializer to create a multipart.
    fn multipart(self) -> Result<Self::MultipartSerializer, Self::Error>;
}

/// A type used to serialize a multipart body to an HTTP request
pub trait MultipartSerializer: Sized {
    /// The result of this multipart call
    type Ok;
    /// The error returned if some part of the multipart fails to serialize
    type Error;

    /// Adds json content to this multipart.
    fn add_json<T: Serialize>(
        self,
        value: &T,
        name: &str,
        file_name: Option<&str>,
    ) -> Result<Self, Self::Error>;

    /// Adds javascript content to this multipart.
    fn add_javascript(
        self,
        value: &str,
        name: &str,
        file_name: Option<&str>,
    ) -> Result<Self, Self::Error> {
        self.add_content(value.as_bytes(), ContentType::Javascript, name, file_name)
    }

    /// Adds a javascript module to this multipart.
    fn add_javascript_module(
        self,
        value: &str,
        name: &str,
        file_name: Option<&str>,
    ) -> Result<Self, Self::Error> {
        self.add_content(
            value.as_bytes(),
            ContentType::JavascriptModule,
            name,
            file_name,
        )
    }

    /// Adds plain text content to this multipart.
    fn add_plain_text(
        self,
        value: &str,
        name: &str,
        file_name: Option<&str>,
    ) -> Result<Self, Self::Error> {
        self.add_content(value.as_bytes(), ContentType::PlainText, name, file_name)
    }

    /// Adds wasm content to this multipart.
    fn add_wasm(
        self,
        bytes: &[u8],
        name: &str,
        file_name: Option<&str>,
    ) -> Result<Self, Self::Error> {
        self.add_content(bytes, ContentType::Wasm, name, file_name)
    }

    /// Adds an octet stream to this multipart.
    fn add_octet_stream(
        self,
        bytes: &[u8],
        name: &str,
        file_name: Option<&str>,
    ) -> Result<Self, Self::Error> {
        self.add_content(bytes, ContentType::OctetStream, name, file_name)
    }

    /// Adds content of the specified part to this multipart.
    fn add_content(
        self,
        bytes: &[u8],
        content_type: ContentType,
        name: &str,
        file_name: Option<&str>,
    ) -> Result<Self, Self::Error>;

    /// Finishes serializing the multipart.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

impl Content for () {
    #[inline]
    fn serialize<C: ContentSerializer>(&self, serializer: C) -> Result<C::Ok, C::Error> {
        serializer.empty()
    }
}

/// Represents content that can be deserialized from the body of an endpoint response.
pub trait Response: Sized {
    /// Deserializes the content using the given ContentDeserializer
    fn deserialize<D: ContentDeserializer>(deserializer: D) -> Result<Self, D::Error>;
}

/// A set of response headers that can be read as part of deserialization.
pub trait ResponseHeaders {
    /// Gets the value of the "Expires" header or None if it doesn't exist.
    fn expires(&self) -> Option<&HeaderValue>;
}

/// An async variant of the Response trait, represents content that can be deserialized
/// from the body of an endpoint response asynchronously.
#[async_trait::async_trait]
#[cfg(feature = "async-trait")]
pub trait AsyncResponse: Sized {
    /// Deserializes the content asynchronously using the given ContentDeserializer
    async fn deserialize<D: AsyncContentDeserializer>(deserializer: D) -> Result<Self, D::Error>;
}

/// A deserializer that can be provided to a response to deserialize it in a specific way.
pub trait ContentDeserializer {
    /// An error that can be returned as part of deserialization
    type Error;
    /// The type used for reading headers from the response
    type Headers: ResponseHeaders;

    /// Gets headers that can be used as part of deserialization
    fn headers(&self) -> &Self::Headers;

    /// Returns the status code of the response, or None if the status code is unknown.
    fn status_code(&self) -> StatusCode;

    /// Returns the content type of the response, or None if the content type is unknown.
    fn content_type(&self) -> Option<ContentType>;

    /// Reads the plain text content as a UTF-8 string.
    fn plain_text(self) -> Result<String, Self::Error>;

    /// Deserializes the content in json format.
    fn json<D: DeserializeOwned>(self) -> Result<D, Self::Error>;

    /// Deserializes the body as a series of bytes
    fn octet_stream(self) -> Result<Vec<u8>, Self::Error>;

    /// Consumes the deserializer, returning an error that can be used to indicate the response is
    /// unable to process the provided content.
    fn unknown(self) -> Self::Error;
}

/// An async variant of the ContentDeserializer trait, provided to a response to deserialize it
/// in a specific way.
#[async_trait::async_trait]
#[cfg(feature = "async-trait")]
pub trait AsyncContentDeserializer: Send + Sync {
    /// An error that can be returned as part of deserialization
    type Error;
    /// The type used for reading headers from the response
    type Headers: ResponseHeaders;

    /// Gets headers that can be used as part of deserialization
    fn headers(&self) -> &Self::Headers;

    /// Returns the status code of the response, or None if the status code is invalid.
    fn status_code(&self) -> StatusCode;

    /// Returns the content type of the response, or None if the content type is unknown.
    fn content_type(&self) -> Option<ContentType>;

    /// Reads the plain text content as a UTF-8 string.
    async fn plain_text(self) -> Result<String, Self::Error>;

    /// Deserializes the content in json format.
    async fn json<D: DeserializeOwned>(self) -> Result<D, Self::Error>;

    /// Deserializes the body as a series of bytes
    async fn octet_stream(self) -> Result<Vec<u8>, Self::Error>;

    /// Consumes the deserializer, returning an error that can be used to indicate the response is
    /// unable to process the provided content.
    fn unknown(self) -> Self::Error;
}

/// A content type supported by the Cloudflare API.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContentType {
    /// Json content ('application/json')
    Json,
    /// Javascript content ('application/javascript')
    Javascript,
    /// Javascript module content ('application/javascript+module')
    JavascriptModule,
    /// Wasm content ('application/wasm')
    Wasm,
    /// Multipart form data content.
    FormData,
    /// Plain text content ('text/plain')
    PlainText,
    /// Data content ('application/octet-stream')
    OctetStream,
}

impl ContentType {
    /// Returns the mime string for the content type
    pub fn as_mime_str(self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::Javascript => "application/javascript",
            ContentType::JavascriptModule => "application/javascript+module",
            ContentType::Wasm => "application/wasm",
            ContentType::FormData => "multipart/form-data",
            ContentType::PlainText => "text/plain",
            ContentType::OctetStream => "application/octet-stream",
        }
    }
}

/// Configures the API response sort order as either ascending or descending.
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// Used as a parameter to API calls that search for a resource (e.g. DNS records).
/// Tells the API whether to return results that match all search requirements or at least one (any).
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SearchMatch {
    /// Match all search requirements
    All,
    /// Match at least one search requirement
    Any,
}

/// A set of credentials to use for API authentication.
#[derive(Debug, Clone)]
pub enum Credentials {
    /// A user auth key with both a user email and an API key.
    UserAuthKey {
        /// The user email provided through the `X-Auth-Email` header
        email: String,
        /// The API key provided through the `X-Auth-Key` header
        key: String,
    },
    /// An API token provided as `Authorization: Bearer <token>`
    UserAuthToken(String),
    /// A service key provided through the `X-Auth-User-Service` header
    ServiceKey(String),
}

/// The default client API URI used by clients.
pub const CLIENT_API_V4_URI: &str = "https://api.cloudflare.com/client/v4/";

/// A JSON response with a json result body and a status code.
#[derive(Debug)]
pub struct JsonResponse<R, I = ()> {
    pub status: StatusCode,
    pub body: JsonResult<R, I>,
}

/// The result structure json content is returned in.
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonResult<R, I = ()> {
    pub success: bool,
    pub errors: Vec<JsonError>,
    pub result: Option<R>,
    pub result_info: Option<I>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonError {
    pub code: i64,
    pub message: String,
}

impl<R: DeserializeOwned, I: DeserializeOwned> Response for JsonResponse<R, I> {
    fn deserialize<D: ContentDeserializer>(deserializer: D) -> Result<Self, D::Error> {
        let status = deserializer.status_code();
        if let Some(ContentType::Json) = deserializer.content_type() {
            deserializer
                .json()
                .map(|body| JsonResponse { status, body })
        } else {
            Err(deserializer.unknown())
        }
    }
}

#[async_trait::async_trait]
#[cfg(feature = "async-trait")]
impl<R: DeserializeOwned, I: DeserializeOwned> AsyncResponse for JsonResponse<R, I> {
    async fn deserialize<D: AsyncContentDeserializer>(deserializer: D) -> Result<Self, D::Error> {
        let status = deserializer.status_code();
        if let Some(ContentType::Json) = deserializer.content_type() {
            deserializer
                .json()
                .await
                .map(|body| JsonResponse { status, body })
        } else {
            Err(deserializer.unknown())
        }
    }
}
