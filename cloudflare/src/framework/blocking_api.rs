use reqwest::blocking::RequestBuilder;
use std::net::SocketAddr;

use crate::framework::auth::Credentials;
use crate::framework::response::{ApiErrors, ApiFailure, ApiResponse, ApiResult, ApiSuccess};
use crate::framework::{
    auth, auth::AuthClient, endpoint, response, Environment, HttpApiClient, HttpApiClientConfig,
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

// There is no blocking implementation for wasm.
#[cfg(all(feature = "blocking", not(target_arch = "wasm32")))]
// If the response is 200 and parses, return Success.
// If the response is 200 and doesn't parse, return Invalid.
// If the response isn't 200, return Failure, with API errors if they were included.
pub fn map_api_response<ResultType: ApiResult>(
    resp: reqwest::blocking::Response,
) -> ApiResponse<ResultType> {
    let status = resp.status();
    if status.is_success() {
        let parsed: Result<ApiSuccess<ResultType>, reqwest::Error> = resp.json();
        match parsed {
            Ok(api_resp) => Ok(api_resp),
            Err(e) => Err(ApiFailure::Invalid(e)),
        }
    } else {
        let parsed: Result<ApiErrors, reqwest::Error> = resp.json();
        let errors = parsed.unwrap_or_default();
        Err(ApiFailure::Error(status, errors))
    }
}

#[cfg(all(test, feature = "blocking", not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::framework::response::ApiError;
    use std::collections::HashMap;

    #[test]
    fn api_failure_eq() {
        let err1 = ApiFailure::Error(
            reqwest::StatusCode::NOT_FOUND,
            ApiErrors {
                errors: vec![ApiError {
                    code: 1000,
                    message: "some failed".to_owned(),
                    other: HashMap::new(),
                }],
                other: HashMap::new(),
            },
        );
        assert_eq!(err1, err1);

        let err2 = ApiFailure::Error(
            reqwest::StatusCode::NOT_FOUND,
            ApiErrors {
                errors: vec![ApiError {
                    code: 1000,
                    message: "some different thing failed".to_owned(),
                    other: HashMap::new(),
                }],
                other: HashMap::new(),
            },
        );
        assert_ne!(err2, err1);

        let not_real_website = "notavalid:url.evena little";
        let fail = ApiFailure::Invalid(reqwest::blocking::get(not_real_website).unwrap_err());
        assert_eq!(fail, fail);
        assert_ne!(fail, err1);
        assert_ne!(fail, err2);
    }
}
