use crate::framework::client::ClientConfig;
use crate::framework::endpoint::{EndpointSpec, MultipartPart, RequestBody};
use crate::framework::response::ResponseConverter;
use crate::framework::{
    auth::{AuthClient, Credentials},
    response::ApiResponse,
    response::{ApiErrors, ApiFailure, ApiSuccess},
    Environment,
};
use std::borrow::Cow;
use std::net::SocketAddr;

/// A Cloudflare API client that makes requests asynchronously.
// TODO: Rename to AsyncClient?
pub struct Client {
    environment: Environment,
    credentials: Credentials,
    http_client: reqwest::Client,
}

impl AuthClient for reqwest::RequestBuilder {
    fn auth(mut self, credentials: &Credentials) -> Self {
        for (k, v) in credentials.headers() {
            self = self.header(k, v);
        }
        self
    }
}

impl Client {
    pub fn new(
        credentials: Credentials,
        config: ClientConfig,
        environment: Environment,
    ) -> Result<Client, crate::framework::Error> {
        let mut builder = reqwest::Client::builder().default_headers(config.default_headers);

        #[cfg(not(target_arch = "wasm32"))]
        {
            // There is no resolve method in wasm.
            if let Some(address) = config.resolve_ip {
                let url = url::Url::from(&environment);
                builder = builder.resolve(
                    url.host_str()
                        .expect("Environment url should have a hostname"),
                    SocketAddr::new(address, 443),
                );
            }

            // There are no timeouts in wasm. The property is documented as no-op in wasm32.
            builder = builder.timeout(config.http_timeout);
        }

        let http_client = builder.build()?;

        Ok(Client {
            environment,
            credentials,
            http_client,
        })
    }

    //noinspection RsConstantConditionIf
    /// Issue an API request of the given type.
    pub async fn request<Endpoint>(
        &self,
        endpoint: &Endpoint,
    ) -> ApiResponse<Endpoint::ResponseType>
    where
        Endpoint: EndpointSpec + Send + Sync,
        Endpoint::ResponseType: ResponseConverter<Endpoint::JsonResponse>,
    {
        // Build the request
        let mut request = self
            .http_client
            .request(endpoint.method(), endpoint.url(&self.environment));

        if let Some(body) = endpoint.body() {
            match body {
                RequestBody::Json(json) => {
                    request = request.body(json);
                }
                RequestBody::Raw(bytes) => {
                    request = request.body(bytes);
                }
                RequestBody::MultiPart(multipart) => {
                    let mut form = reqwest::multipart::Form::new();
                    for (name, part) in multipart.parts() {
                        match part {
                            MultipartPart::Text(text) => {
                                form = form.text(name, text);
                            }
                            MultipartPart::Bytes(bytes) => {
                                form = form.part(name, reqwest::multipart::Part::bytes(bytes));
                            }
                        }
                    }
                    request = request.multipart(form);
                }
            }
            // Reqwest::RequestBuilder::multipart sets the content type for us.
            match endpoint.content_type() {
                None | Some(Cow::Borrowed("multipart/form-data")) => {}
                Some(content_type) => {
                    request = request.header(reqwest::header::CONTENT_TYPE, content_type.as_ref());
                }
            }
        }

        request = request.auth(&self.credentials);
        let response = request.send().await?;

        // The condition is necessary, even if a warning is present.
        // The constant is overridden in some cases.
        if Endpoint::IS_RAW_BODY {
            map_api_response_raw::<Endpoint>(response).await
        } else {
            map_api_response_json::<Endpoint>(response).await
        }
    }
}

// If the response is 2XX and parses, return Success.
// If the response is 2XX and doesn't parse, return Invalid.
// If the response isn't 2XX, return Failure, with API errors if they were included.
async fn map_api_response_raw<Endpoint>(
    resp: reqwest::Response,
) -> Result<Endpoint::ResponseType, ApiFailure>
where
    Endpoint: EndpointSpec,
    Endpoint::ResponseType: ResponseConverter<Endpoint::JsonResponse>,
{
    let status = resp.status();
    if status.is_success() {
        let bytes = resp.bytes().await.map_err(ApiFailure::Invalid)?.to_vec();
        Ok(Endpoint::ResponseType::from_raw(bytes))
    } else {
        let parsed: Result<ApiErrors, reqwest::Error> = resp.json().await;
        let errors = parsed.unwrap_or_default();
        Err(ApiFailure::Error(status, errors))
    }
}

async fn map_api_response_json<Endpoint>(
    resp: reqwest::Response,
) -> Result<Endpoint::ResponseType, ApiFailure>
where
    Endpoint: EndpointSpec,
    Endpoint::ResponseType: ResponseConverter<Endpoint::JsonResponse>,
{
    let status = resp.status();
    if status.is_success() {
        let parsed: Result<ApiSuccess<Endpoint::JsonResponse>, reqwest::Error> = resp.json().await;
        match parsed {
            Ok(success) => Ok(Endpoint::ResponseType::from_json(success)),
            Err(e) => Err(ApiFailure::Invalid(e)),
        }
    } else {
        let parsed: Result<ApiErrors, reqwest::Error> = resp.json().await;
        let errors = parsed.unwrap_or_default();
        Err(ApiFailure::Error(status, errors))
    }
}

// TODO: Refactor this to test the blocking_api as well
#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::auth::Credentials;
    use crate::framework::client::ClientConfig;
    use crate::framework::endpoint::RequestBody;
    use crate::framework::endpoint::{serialize_query, EndpointSpec};
    use crate::framework::response::{ApiFailure, ApiResult, ApiSuccess};
    use crate::framework::Environment;
    use mockito::{Matcher, Server};
    use regex;
    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use tokio;

    //region Endpoint that returns JSON (ApiSuccess).
    #[derive(Debug)]
    struct DummyJsonEndpoint;

    #[derive(Debug, Deserialize)]
    struct DummyJsonResponse {
        message: String,
    }

    impl ApiResult for DummyJsonResponse {}

    impl EndpointSpec for DummyJsonEndpoint {
        type JsonResponse = DummyJsonResponse;
        type ResponseType = ApiSuccess<Self::JsonResponse>;

        fn method(&self) -> reqwest::Method {
            reqwest::Method::GET
        }

        fn path(&self) -> String {
            "/dummy/json".into()
        }
    }
    //endregion

    //region Endpoint that returns raw bytes.
    #[derive(Debug)]
    struct DummyRawEndpoint;

    impl EndpointSpec for DummyRawEndpoint {
        const IS_RAW_BODY: bool = true;
        type JsonResponse = ();
        type ResponseType = Vec<u8>;

        fn method(&self) -> reqwest::Method {
            reqwest::Method::GET
        }

        fn path(&self) -> String {
            "/dummy/raw".into()
        }
    }
    //endregion

    //region Endpoint that returns nothing.
    #[derive(Debug)]
    struct DummyNothingEndpoint;

    impl EndpointSpec for DummyNothingEndpoint {
        type JsonResponse = ();
        type ResponseType = ApiSuccess<Self::JsonResponse>;

        fn method(&self) -> reqwest::Method {
            reqwest::Method::GET
        }

        fn path(&self) -> String {
            "/dummy/nothing".into()
        }
    }
    //endregion

    //region Endpoint that sends a JSON request.
    #[derive(Debug)]
    struct DummyJsonRequestEndpoint;

    impl EndpointSpec for DummyJsonRequestEndpoint {
        type JsonResponse = ();
        type ResponseType = ApiSuccess<Self::JsonResponse>;

        fn method(&self) -> reqwest::Method {
            reqwest::Method::POST
        }

        fn path(&self) -> String {
            "/dummy/json".into()
        }

        fn body(&self) -> Option<RequestBody> {
            Some(RequestBody::Json(json!({"key": "value"}).to_string()))
        }
    }
    //endregion

    //region Endpoint that sends raw bytes.
    #[derive(Debug)]
    struct DummyRawRequestEndpoint;

    impl EndpointSpec for DummyRawRequestEndpoint {
        const IS_RAW_BODY: bool = true;
        type JsonResponse = ();
        type ResponseType = Vec<u8>;

        fn method(&self) -> reqwest::Method {
            reqwest::Method::POST
        }

        fn path(&self) -> String {
            "/dummy/raw".into()
        }

        fn body(&self) -> Option<RequestBody> {
            Some(RequestBody::Raw(b"raw content".to_vec()))
        }
    }
    //endregion

    //region Endpoint that sends a multipart request.
    #[derive(Debug)]
    struct DummyMultipartEndpoint;

    impl EndpointSpec for DummyMultipartEndpoint {
        type JsonResponse = ();
        type ResponseType = ApiSuccess<Self::JsonResponse>;

        fn method(&self) -> reqwest::Method {
            reqwest::Method::POST
        }

        fn path(&self) -> String {
            "/dummy/multipart".into()
        }

        fn body(&self) -> Option<RequestBody> {
            Some(RequestBody::MultiPart(&DummyMultipart))
        }
    }

    struct DummyMultipart;

    impl crate::framework::endpoint::MultipartBody for DummyMultipart {
        fn parts(&self) -> Vec<(String, MultipartPart)> {
            vec![("key".into(), MultipartPart::Text("value".into()))]
        }
    }
    //endregion

    //region Endpoint that sends a request with query parameters.
    #[derive(Debug)]
    struct DummyJsonRequestWithQueryEndpoint;

    #[derive(Debug, Serialize)]
    struct DummyJsonRequestWithQueryParams {
        key: String,
    }

    impl EndpointSpec for DummyJsonRequestWithQueryEndpoint {
        type JsonResponse = ();
        type ResponseType = ApiSuccess<Self::JsonResponse>;

        fn method(&self) -> reqwest::Method {
            reqwest::Method::POST
        }

        fn path(&self) -> String {
            "/dummy/json".into()
        }

        fn query(&self) -> Option<String> {
            serialize_query(&DummyJsonRequestWithQueryParams {
                key: "value".into(),
            })
        }
    }
    //endregion

    fn create_test_client(url: String) -> Client {
        let environment = Environment::Custom(url);
        let credentials = Credentials::UserAuthToken {
            token: "dummy".into(),
        };
        let config = ClientConfig::default();
        Client::new(credentials, config, environment).unwrap()
    }

    /// Test that the client can successfully request a JSON endpoint.
    #[tokio::test]
    async fn test_json_endpoint_success() {
        let body = json!({
            "result": {"message": "Hello, World!"},
            "result_info": null,
            "messages": [],
            "errors": [],
            "success": true
        });

        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/dummy/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body.to_string())
            .match_header("content-type", Matcher::Missing)
            .match_query(Matcher::Missing)
            .match_body(Matcher::Missing)
            .create();

        let client = create_test_client(server.url());
        let response = client.request(&DummyJsonEndpoint).await;

        mock.assert();
        let response = response.unwrap();
        assert_eq!(response.result.message, "Hello, World!");
        assert_eq!(response.result_info, None);
        assert!(response.messages.is_empty());
        assert!(response.errors.is_empty());
    }

    /// Test that the client can successfully request a raw endpoint.
    #[tokio::test]
    async fn test_raw_endpoint_success() {
        let raw_body = b"raw content".to_vec();

        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/dummy/raw")
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(raw_body.clone())
            .match_header("content-type", Matcher::Missing)
            .match_query(Matcher::Missing)
            .match_body(Matcher::Missing)
            .create();

        let client = create_test_client(server.url());
        let response = client.request(&DummyRawEndpoint).await.unwrap();

        mock.assert();
        assert_eq!(response, raw_body);
    }

    /// Test that the client can handle an endpoint that returns an error.
    #[tokio::test]
    async fn test_endpoint_failure() {
        let body = json!({
            "errors": [{"code": 123, "message": "Something went wrong", "other": {}}],
            "other": {}
        });

        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/dummy/json")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(body.to_string())
            .match_header("content-type", Matcher::Missing)
            .match_query(Matcher::Missing)
            .match_body(Matcher::Missing)
            .create();

        let client = create_test_client(server.url());
        let result = client.request(&DummyJsonEndpoint).await;

        mock.assert();
        assert!(result.is_err());
        if let Err(ApiFailure::Error(status, errors)) = result {
            assert_eq!(status.as_u16(), 400);
            assert!(!errors.errors.is_empty());
            assert_eq!(errors.errors[0].code, 123);
        } else {
            panic!("Expected error result");
        }
    }

    /// Test that the client can handle an endpoint that returns nothing.
    #[tokio::test]
    async fn test_nothing_endpoint_success() {
        let body = json!({
            "result": null,
            "result_info": null,
            "messages": [],
            "errors": [],
            "success": true
        });

        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/dummy/nothing")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body.to_string())
            .match_header("content-type", Matcher::Missing)
            .match_query(Matcher::Missing)
            .match_body(Matcher::Missing)
            .create();

        let client = create_test_client(server.url());
        let response = client.request(&DummyNothingEndpoint).await;

        mock.assert();
        let response = response.unwrap();
        assert!(matches!(response.result, ()));
        assert_eq!(response.result_info, None);
        assert!(response.messages.is_empty());
        assert!(response.errors.is_empty());
    }

    /// Test that the client can successfully send a JSON request.
    #[tokio::test]
    async fn test_json_body_success() {
        let body = json!({
            "result": null,
            "result_info": null,
            "messages": [],
            "errors": [],
            "success": true
        });

        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/dummy/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body.to_string())
            .match_header("content-type", "application/json")
            .match_query(Matcher::Missing)
            .match_body(Matcher::Json(json!({"key": "value"})))
            .create();

        let client = create_test_client(server.url());
        let _ = client.request(&DummyJsonRequestEndpoint).await;

        mock.assert();
    }

    /// Test that the client can successfully send a raw request.
    #[tokio::test]
    async fn test_raw_body_success() {
        let raw_body = b"raw content".to_vec();

        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/dummy/raw")
            .with_status(200)
            .with_header("content-type", "application/octet-stream")
            .with_body(raw_body.clone())
            .match_header("content-type", "application/octet-stream")
            .match_query(Matcher::Missing)
            .match_body(raw_body)
            .create();

        let client = create_test_client(server.url());
        let _ = client.request(&DummyRawRequestEndpoint).await;

        mock.assert();
    }

    /// Test that the client can successfully send a multipart request.
    #[tokio::test]
    async fn test_multipart_body_success() {
        let body = json!({
            "result": null,
            "result_info": null,
            "messages": [],
            "errors": [],
            "success": true
        });

        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/dummy/multipart")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body.to_string())
            .match_header(
                "content-type",
                Matcher::Regex("multipart/form-data; boundary=.*".into()),
            )
            .match_query(Matcher::Missing)
            .match_request(|req| {
                let body = req.body().unwrap().to_vec();
                let body = String::from_utf8_lossy(&body);

                let re = Regex::new(
                    r#"^--.*\s+Content-Disposition: form-data; name="key"\s+\s+value\s+--.*\s*$"#,
                )
                .unwrap();
                re.is_match(&body)
            })
            .create();

        let client = create_test_client(server.url());
        let _ = client.request(&DummyMultipartEndpoint).await;

        mock.assert();
    }

    /// Test that the client can successfully send a request with query parameters.
    #[tokio::test]
    async fn test_query_parameters_success() {
        let body = json!({
            "result": null,
            "result_info": null,
            "messages": [],
            "errors": [],
            "success": true
        });

        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/dummy/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body.to_string())
            .match_header("content-type", Matcher::Missing)
            .match_query(Matcher::UrlEncoded("key".into(), "value".into()))
            .match_body(Matcher::Missing)
            .create();

        let client = create_test_client(server.url());
        let _ = client.request(&DummyJsonRequestWithQueryEndpoint).await;

        mock.assert();
    }
}
