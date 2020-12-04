use reqwest::blocking::RequestBuilder;
use serde::Serialize;

use crate::framework::auth::Credentials;
use crate::framework::reqwest_adaptors::match_reqwest_method;
use crate::framework::{
    apiclient::ApiClient, auth, auth::AuthClient, endpoint, response, response::map_api_response,
    Environment, HttpApiClient, HttpApiClientConfig,
};

impl HttpApiClient {
    pub fn new(
        credentials: auth::Credentials,
        config: HttpApiClientConfig,
        environment: Environment,
    ) -> anyhow::Result<HttpApiClient> {
        let http_client = reqwest::blocking::Client::builder()
            .timeout(config.http_timeout)
            .default_headers(config.default_headers)
            .build()?;

        Ok(HttpApiClient {
            environment,
            credentials,
            http_client,
        })
    }
}

// TODO: This should probably just implement request for the Reqwest client itself :)
// TODO: It should also probably be called `ReqwestApiClient` rather than `HttpApiClient`.
impl<'a> ApiClient for HttpApiClient {
    /// Synchronously send a request to the Cloudflare API.
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn endpoint::Endpoint<ResultType, QueryType, BodyType>,
    ) -> response::ApiResponse<ResultType>
    where
        ResultType: response::ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        // Build the request
        let mut request = self
            .http_client
            .request(
                match_reqwest_method(endpoint.method()),
                endpoint.url(&self.environment),
            )
            .query(&endpoint.query());

        if let Some(body) = endpoint.body() {
            request = request.body(serde_json::to_string(&body).unwrap());
            request = request.header(reqwest::header::CONTENT_TYPE, endpoint.content_type());
        }

        request = request.auth(&self.credentials);

        let response = request.send()?;

        map_api_response(response)
    }
}

impl AuthClient for RequestBuilder {
    fn auth(mut self, credentials: &Credentials) -> Self {
        for (k, v) in credentials.headers() {
            self = self.header(k, v);
        }
        self
    }
}
