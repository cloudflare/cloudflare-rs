use crate::{Credentials, Endpoint, CLIENT_API_V4_URI};
use url::Url;

/// A basic builder that contains credentials and a base API URL for use with constructing requests
/// for Endpoints.
pub struct RequestBuilder {
    /// The credentials to use for requests
    pub credentials: Credentials,
    /// The API URL to make requests to
    pub api_url: Url,
}

impl RequestBuilder {
    /// Creates a new builder with the provided credentials and the default client API url.
    pub fn new(credentials: Credentials) -> Self {
        Self {
            credentials,
            api_url: CLIENT_API_V4_URI.parse().expect("default API URI is valid"),
        }
    }

    /// Attempts to create a request using this builder and the specified endpoint. If the API Url
    /// cannot be joined with the endpoint's path, this returns a URL parse error.
    pub fn try_build<'a, E: Endpoint + 'a>(
        &'a self,
        endpoint: &'a E,
    ) -> Result<Request<'a, E>, url::ParseError> {
        Ok(Request {
            url: self.api_url.join(&endpoint.path())?,
            credentials: &self.credentials,
            query: endpoint.query(),
            body: endpoint.body(),
        })
    }

    /// Creates a request using this builder and the specified endpoint.
    ///
    /// # Panics
    ///
    /// If the API Url cannot be joined with the endpoint's path.
    pub fn build<'a, E: Endpoint + 'a>(&'a self, endpoint: &'a E) -> Request<'a, E> {
        self.try_build(endpoint)
            .expect("failed to join base API url to endpoint path")
    }
}

/// Pairs an endpoint and a request builder's variables together
pub struct Request<'a, E: Endpoint + 'a> {
    /// The URL to use for this request. This does not include the query.
    pub url: Url,
    /// The credentials to use for this request
    pub credentials: &'a Credentials,
    /// The query for this endpoint
    pub query: &'a E::Query,
    /// The body to use for this request
    pub body: &'a E::Body,
}

/// Provides extensions for making requests using endpoints
pub trait EndpointExt: Endpoint {
    /// Constructs a `Request` using this endpoint and the given `RequestBuilder`.
    ///
    /// # Panics
    ///
    /// If the base API URL cannot be joined with this endpoint path and query.
    fn request<'a>(&'a self, builder: &'a RequestBuilder) -> Request<'a, Self>;

    /// Attempts to construct a `Request` using this endpoint and the given `RequestBuilder`,
    /// returning a [`url::ParseError`] if the base API URL cannot be joined with this endpoint
    /// path and query.
    fn try_request<'a>(
        &'a self,
        builder: &'a RequestBuilder,
    ) -> Result<Request<'a, Self>, url::ParseError>;
}

impl<E: Endpoint> EndpointExt for E {
    fn request<'a>(&'a self, builder: &'a RequestBuilder) -> Request<'a, Self> {
        builder.build(self)
    }
    fn try_request<'a>(
        &'a self,
        builder: &'a RequestBuilder,
    ) -> Result<Request<'a, Self>, url::ParseError> {
        builder.try_build(self)
    }
}
