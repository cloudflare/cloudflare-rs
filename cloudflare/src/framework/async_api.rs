use crate::framework::endpoint::{EndpointSpec, MultipartPart, RequestBody};
use crate::framework::response::ResponseConverter;
use crate::framework::{
    auth::{AuthClient, Credentials},
    response::ApiResponse,
    response::{ApiErrors, ApiFailure, ApiSuccess},
    Environment, HttpApiClientConfig,
};
use std::net::SocketAddr;

/// A Cloudflare API client that makes requests asynchronously.
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
        config: HttpApiClientConfig,
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

                    //TODO: Maybe check if the content type is correct somewhere?
                }
            }
            // Reqwest::RequestBuilder::multipart sets the content type for us.
            if endpoint.content_type() != "multipart/form-data" {
                request = request.header(
                    reqwest::header::CONTENT_TYPE,
                    endpoint.content_type().as_ref(),
                );
            }
        }

        request = request.auth(&self.credentials);
        println!("{:?}", request);
        let response = request.send().await?;

        // The condition is necessary, even if a warning is present.
        // The constant is overridden in some cases.
        if Endpoint::IS_RAW_BODY {
            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|ct| ct.to_str().ok())
                .unwrap_or("");
            assert_eq!(content_type, "application/octet-stream");

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
