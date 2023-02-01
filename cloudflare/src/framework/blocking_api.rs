use reqwest::blocking::RequestBuilder;
use std::net::SocketAddr;

use crate::framework::auth::Credentials;
use crate::framework::{
    auth, auth::AuthClient, endpoint, response, response::map_api_response, Environment,
    HttpApiClient, HttpApiClientConfig,
};

impl HttpApiClient {
    pub fn new(
        credentials: auth::Credentials,
        config: HttpApiClientConfig,
        environment: Environment,
    ) -> Result<HttpApiClient, crate::framework::Error> {
        let mut builder = reqwest::blocking::Client::builder()
            .timeout(config.http_timeout)
            .default_headers(config.default_headers);

        if let Some(address) = config.resolve_ip {
            let url = url::Url::from(&environment);
            builder = builder.resolve(
                url.host_str()
                    .expect("Environment url should have a hostname"),
                SocketAddr::new(address, 443),
            );
        }
        let http_client = builder.build()?;

        Ok(HttpApiClient {
            environment,
            credentials,
            http_client,
        })
    }

    // TODO: This should probably just implement request for the Reqwest client itself :)
    // TODO: It should also probably be called `ReqwestApiClient` rather than `HttpApiClient`.
    /// Synchronously send a request to the Cloudflare API.
    pub fn request<ResultType>(
        &self,
        endpoint: &dyn endpoint::Endpoint<ResultType>,
    ) -> response::ApiResponse<ResultType>
    where
        ResultType: response::ApiResult,
    {
        // Build the request
        let mut request = self
            .http_client
            .request(endpoint.method(), endpoint.url(&self.environment));

        if let Some(body) = endpoint.body() {
            request = request.body(body);
            request = request.header(
                reqwest::header::CONTENT_TYPE,
                endpoint.content_type().as_ref(),
            );
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
