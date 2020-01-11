use crate::framework::Environment;
use serde::Serialize;
use url::Url;

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

/// Represents an endpoint in the [Cloudflare API](api.cloudflare.com)
/// Endpoints accept requests with a certain QueryType and BodyType. They return a certain
/// ResponseType.
///
/// If the endpoint is defined to return a JSON response, simply define a Rust struct `Foo` that
/// matches that JSON, ensure `impl ApiResult for Foo` and define your endpoint with
/// `ResultType = Foo`.
///
/// If the endpoint is defined to respond with bytes, simply define your endpoint with
/// `ResultType = endpoint::Binary`.
pub trait Endpoint<ResultType = (), QueryType = (), BodyType = ()>
where
    QueryType: Serialize,
    BodyType: Serialize,
{
    fn method(&self) -> Method;
    fn path(&self) -> String;
    fn query(&self) -> Option<QueryType> {
        None
    }
    fn body(&self) -> Option<BodyType> {
        None
    }
    fn url(&self, environment: &Environment) -> Url {
        Url::from(environment).join(&self.path()).unwrap()
    }
    fn content_type(&self) -> String {
        "application/json".to_owned()
    }
}

/// If a Cloudflare API endpoint returns bytes instead of JSON, the `Endpoint` object should be
/// declared with `ResultType = Binary` and should be accessed with `request_binary`, not `request`.
///
/// # Examples
///
/// ```
/// use cloudflare::framework::{
///     async_api::ApiClient,
///     endpoint::{Binary, Endpoint, Method},
/// };
///
/// struct BinaryEndpoint {
///     body_contents: String,
/// }
///
/// impl Endpoint<Binary, (), String> for BinaryEndpoint {
///     fn method(&self) -> Method {
///         Method::Post
///     }
///     fn path(&self) -> String {
///         "some/path".to_owned()
///     }
///     fn body(&self) -> Option<String> {
///         Some(self.body_contents.clone())
///     }
/// }
///
/// fn request_binary<ApiClientType: ApiClient>(api_client: &ApiClientType) {
///     let req = BinaryEndpoint {
///         body_contents: "asdf".to_owned(),
///     };
///     let _resp = api_client.request_binary(&req);
/// }
/// ```
///
/// ```compile_fail
/// use cloudflare::framework::{
///     async_api::ApiClient,
///     endpoint::{Binary, Endpoint, Method},
/// };
///
/// struct BinaryEndpoint {
///     body_contents: String,
/// }
///
/// impl Endpoint<Binary, (), String> for BinaryEndpoint {
///     fn method(&self) -> Method {
///         Method::Post
///     }
///     fn path(&self) -> String {
///         "some/path".to_owned()
///     }
///     fn body(&self) -> Option<String> {
///         Some(self.body_contents.clone())
///     }
/// }
///
/// fn request_binary_into_json<ApiClientType: ApiClient>(api_client: &ApiClientType) {
///     let req = BinaryEndpoint {
///         body_contents: "asdf".to_owned(),
///     };
///
///     // This won't compile, because you can't use the `request` method with a binary endpoint.
///     let _resp = api_client.request(&req);
/// }
/// ```
///
/// ```compile_fail
/// use cloudflare::framework::{
///     async_api::ApiClient,
///     endpoint::{Binary, Endpoint, Method},
/// };
/// use cloudflare::endpoints::account::ListAccounts;
///
/// fn request_json_into_binary<ApiClientType: ApiClient>(api_client: &ApiClientType) {
///     let req = ListAccounts { params: None };
///
///     // This won't compile, because you can't use the `request_binary` method with a JSON
///     // endpoint.
///     let _resp = api_client.request_binary(&req);
/// }
/// ```
pub enum Binary {}
