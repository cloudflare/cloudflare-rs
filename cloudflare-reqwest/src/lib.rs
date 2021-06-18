use async_trait::async_trait;
use cloudflare::{
    request, AsyncContentDeserializer, AsyncResponse, Content, ContentDeserializer,
    ContentSerializer, ContentType, Credentials, Endpoint, HeaderValue, Method, Response,
    ResponseHeaders, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use thiserror::Error;

#[cfg(feature = "blocking")]
use reqwest::blocking;

use reqwest::header::HeaderMap;

#[repr(transparent)]
struct HeaderReader(HeaderMap);
impl ResponseHeaders for HeaderReader {
    fn expires(&self) -> Option<&HeaderValue> {
        self.0.get(reqwest::header::EXPIRES)
    }
}

const fn cf_to_reqwest_method(m: cloudflare::Method) -> reqwest::Method {
    match m {
        Method::Get => reqwest::Method::GET,
        Method::Post => reqwest::Method::POST,
        Method::Put => reqwest::Method::PUT,
        Method::Patch => reqwest::Method::PATCH,
        Method::Delete => reqwest::Method::DELETE,
    }
}

fn get_content_type(map: &HeaderMap) -> Option<ContentType> {
    let header = map.get(reqwest::header::CONTENT_TYPE)?.to_str().ok()?;
    Some(match header {
        "application/json" => ContentType::Json,
        "application/javascript" => ContentType::Javascript,
        "application/javascript+module" => ContentType::JavascriptModule,
        "application/wasm" => ContentType::Wasm,
        "multipart/form-data" => ContentType::FormData,
        "application/octet-stream" => ContentType::OctetStream,
        text if text.starts_with("text/plain") => ContentType::PlainText,
        _ => return None,
    })
}

/// An error occured while creating the request.
#[derive(Error, Debug)]
pub enum RequestError {
    #[error("An error occured while serializing JSON content")]
    /// An error occured while serializing JSON content
    Json(#[from] serde_json::Error),
    #[error("An error occured while processing the request")]
    /// An error occured while processing the request
    Reqwest(#[from] reqwest::Error),
}

/// Provides the `call` function to convert a `cloudflare::request::Request` into a
/// `reqwest::RequestBuilder` or `reqwest::blocking::RequestBuilder`.
pub trait CallClientExt<C> {
    /// The resulting request builder that can be used to customize the request
    type Request;

    /// Converts the request into a reqwest request
    fn call(self, client: &C) -> Result<Self::Request, RequestError>;
}

impl<E: Endpoint> CallClientExt<reqwest::Client> for request::Request<'_, E>
where
    E::Query: Serialize,
    E::Body: Content,
{
    type Request = reqwest::RequestBuilder;

    fn call(self, client: &reqwest::Client) -> Result<Self::Request, RequestError> {
        let method = cf_to_reqwest_method(E::METHOD);
        let mut builder = client.request(method, self.url);
        builder = match self.credentials {
            Credentials::UserAuthKey { email, key } => builder
                .header("X-Auth-Email", email)
                .header("X-Auth-Key", key),
            Credentials::ServiceKey(key) => builder.header("X-Auth-User-Service", key),
            Credentials::UserAuthToken(token) => builder.bearer_auth(token),
        };
        builder = builder.query(self.query);

        use reqwest::multipart::{Form, Part};

        struct ReqwestMultipartSerializer {
            builder: reqwest::RequestBuilder,
            form: Form,
        }
        impl cloudflare::MultipartSerializer for ReqwestMultipartSerializer {
            type Ok = reqwest::RequestBuilder;
            type Error = RequestError;

            fn add_json<T: Serialize>(
                self,
                value: &T,
                name: &str,
                file_name: Option<&str>,
            ) -> Result<Self, Self::Error> {
                let serialized = serde_json::to_vec(value)?;
                self.add_content(&serialized, ContentType::Json, name, file_name)
            }

            fn add_content(
                self,
                bytes: &[u8],
                content: ContentType,
                name: &str,
                file_name: Option<&str>,
            ) -> Result<Self, Self::Error> {
                let part = Part::bytes(bytes.to_owned())
                    .mime_str(content.as_mime_str())
                    .expect("valid mime string for content");
                let part = if let Some(file) = file_name {
                    part.file_name(file.to_owned())
                } else {
                    part
                };
                let ReqwestMultipartSerializer { builder, form } = self;
                Ok(ReqwestMultipartSerializer {
                    builder,
                    form: form.part(name.to_owned(), part),
                })
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                Ok(self.builder.multipart(self.form))
            }
        }

        struct ReqwestSerializer(reqwest::RequestBuilder);
        impl ContentSerializer for ReqwestSerializer {
            type Ok = reqwest::RequestBuilder;
            type Error = RequestError;
            type MultipartSerializer = ReqwestMultipartSerializer;

            fn empty(self) -> Result<Self::Ok, Self::Error> {
                Ok(self.0)
            }

            fn json<T: Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
                let body = serde_json::to_vec(value)?;
                Ok(self
                    .0
                    .header(
                        reqwest::header::CONTENT_TYPE,
                        HeaderValue::from_static("application/json"),
                    )
                    .body(body))
            }

            fn multipart(self) -> Result<Self::MultipartSerializer, Self::Error> {
                Ok(ReqwestMultipartSerializer {
                    builder: self.0,
                    form: reqwest::multipart::Form::new(),
                })
            }
        }

        self.body.serialize(ReqwestSerializer(builder))
    }
}

#[cfg(all(feature = "blocking", not(target_arch = "wasm32")))]
impl<E: Endpoint> CallClientExt<blocking::Client> for request::Request<'_, E>
where
    E::Query: Serialize,
    E::Body: Content,
{
    type Request = blocking::RequestBuilder;

    fn call(self, client: &blocking::Client) -> Result<Self::Request, RequestError> {
        let method = cf_to_reqwest_method(E::METHOD);
        let mut builder = client.request(method, self.url);
        builder = match self.credentials {
            Credentials::UserAuthKey { email, key } => builder
                .header("X-Auth-Email", email)
                .header("X-Auth-Key", key),
            Credentials::ServiceKey(key) => builder.header("X-Auth-User-Service", key),
            Credentials::UserAuthToken(token) => builder.bearer_auth(token),
        };
        builder = builder.query(self.query);

        use reqwest::blocking::multipart::{Form, Part};

        struct ReqwestMultipartSerializer {
            builder: reqwest::blocking::RequestBuilder,
            form: Form,
        }
        impl cloudflare::MultipartSerializer for ReqwestMultipartSerializer {
            type Ok = reqwest::blocking::RequestBuilder;
            type Error = RequestError;

            fn add_json<T: Serialize>(
                self,
                value: &T,
                name: &str,
                file_name: Option<&str>,
            ) -> Result<Self, Self::Error> {
                let serialized = serde_json::to_vec(value)?;
                self.add_content(&serialized, ContentType::Json, name, file_name)
            }

            fn add_content(
                self,
                bytes: &[u8],
                content: ContentType,
                name: &str,
                file_name: Option<&str>,
            ) -> Result<Self, Self::Error> {
                let part = Part::bytes(bytes.to_owned())
                    .mime_str(content.as_mime_str())
                    .expect("valid mime string for content");
                let part = if let Some(file) = file_name {
                    part.file_name(file.to_owned())
                } else {
                    part
                };
                let ReqwestMultipartSerializer { builder, form } = self;
                Ok(ReqwestMultipartSerializer {
                    builder,
                    form: form.part(name.to_owned(), part),
                })
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                Ok(self.builder.multipart(self.form))
            }
        }

        struct ReqwestSerializer(reqwest::blocking::RequestBuilder);
        impl ContentSerializer for ReqwestSerializer {
            type Ok = reqwest::blocking::RequestBuilder;
            type Error = RequestError; // instead the request fails when the user tries to send it
            type MultipartSerializer = ReqwestMultipartSerializer;

            fn empty(self) -> Result<Self::Ok, Self::Error> {
                Ok(self.0)
            }

            fn json<T: Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
                let body = serde_json::to_vec(value)?;
                Ok(self
                    .0
                    .header(
                        reqwest::header::CONTENT_TYPE,
                        HeaderValue::from_static("application/json"),
                    )
                    .body(body))
            }

            fn multipart(self) -> Result<Self::MultipartSerializer, Self::Error> {
                Ok(ReqwestMultipartSerializer {
                    builder: self.0,
                    form: reqwest::blocking::multipart::Form::new(),
                })
            }
        }

        self.body.serialize(ReqwestSerializer(builder))
    }
}

/// An error occured while reading the response
#[derive(Error, Debug)]
pub enum ResponseError<T: Debug> {
    #[error("An error occured while processing the response")]
    /// A reqwest error occured while processing the response
    Reqwest(#[from] reqwest::Error),
    #[error("An error occured while deserializing JSON content")]
    /// An error occured while deserializing JSON content
    Json(#[from] serde_json::Error),
    #[error("The response was in an unknown format")]
    /// The response could not be read in the specified endpoint's Response format.
    /// The original reqwest Response should be provided for introspection and debugging
    UnknownContent(T),
}

#[async_trait]
pub trait ReadResponseExt: Debug + Sized {
    async fn read<E: Endpoint>(self) -> Result<E::Response, ResponseError<Self>>
    where
        E::Response: AsyncResponse;
}

#[async_trait]
impl ReadResponseExt for reqwest::Response {
    async fn read<E: Endpoint>(self) -> Result<E::Response, ResponseError<Self>>
    where
        E::Response: AsyncResponse,
    {
        struct Reader(reqwest::Response);
        #[async_trait]
        impl AsyncContentDeserializer for Reader {
            type Error = ResponseError<reqwest::Response>;
            type Headers = HeaderReader;

            fn headers(&self) -> &Self::Headers {
                let map = self.0.headers();
                // HeaderReader is a transparent wrapper over HeaderMap, so this cast is safe
                unsafe { &*((map as *const HeaderMap) as *const HeaderReader) }
            }

            fn status_code(&self) -> StatusCode {
                self.0.status()
            }

            fn content_type(&self) -> Option<ContentType> {
                get_content_type(self.0.headers())
            }

            async fn plain_text(self) -> Result<String, Self::Error> {
                self.0.text().await.map_err(Into::into)
            }
            async fn json<D: DeserializeOwned>(self) -> Result<D, Self::Error> {
                let bytes = self.0.bytes().await?;
                let result = serde_json::from_slice(&bytes)?;
                Ok(result)
            }
            async fn octet_stream(self) -> Result<Vec<u8>, Self::Error> {
                self.0
                    .bytes()
                    .await
                    .map(|bytes| bytes.to_vec())
                    .map_err(Into::into)
            }
            fn unknown(self) -> Self::Error {
                ResponseError::UnknownContent(self.0)
            }
        }

        AsyncResponse::deserialize(Reader(self)).await
    }
}

#[cfg(all(feature = "blocking", not(target_arch = "wasm32")))]
pub trait ReadBlockingResponseExt: Debug + Sized {
    fn read<E: Endpoint>(self) -> Result<E::Response, ResponseError<Self>>
    where
        E::Response: Response;
}

#[cfg(all(feature = "blocking", not(target_arch = "wasm32")))]
impl ReadBlockingResponseExt for reqwest::blocking::Response {
    fn read<E: Endpoint>(self) -> Result<E::Response, ResponseError<Self>>
    where
        E::Response: Response,
    {
        struct Reader(reqwest::blocking::Response);
        impl ContentDeserializer for Reader {
            type Error = ResponseError<reqwest::blocking::Response>;
            type Headers = HeaderReader;

            fn headers(&self) -> &Self::Headers {
                let map = self.0.headers();
                // HeaderReader is a transparent wrapper over HeaderMap, so this cast is safe
                unsafe { &*((map as *const HeaderMap) as *const HeaderReader) }
            }

            fn status_code(&self) -> StatusCode {
                self.0.status()
            }

            fn content_type(&self) -> Option<ContentType> {
                get_content_type(self.0.headers())
            }

            fn plain_text(self) -> Result<String, Self::Error> {
                self.0.text().map_err(Into::into)
            }
            fn json<D: DeserializeOwned>(self) -> Result<D, Self::Error> {
                let bytes = self.0.bytes()?;
                let result = serde_json::from_slice(&bytes)?;
                Ok(result)
            }
            fn octet_stream(self) -> Result<Vec<u8>, Self::Error> {
                self.0
                    .bytes()
                    .map(|bytes| bytes.to_vec())
                    .map_err(Into::into)
            }
            fn unknown(self) -> Self::Error {
                ResponseError::UnknownContent(self.0)
            }
        }

        Response::deserialize(Reader(self))
    }
}

/// An error occured while sending the request
#[derive(Error, Debug)]
pub enum Error<T: Debug> {
    #[error("An error occured while serializing or deserializing JSON content")]
    /// An error occured while serializing or deserializing JSON content
    Json(#[from] serde_json::Error),
    #[error("An error occured while processing the request or response")]
    /// An error occured while processing the request or response
    Reqwest(#[from] reqwest::Error),
    #[error("The response was in an unknown endpoint response format")]
    /// The response was in an unknown endpoint response format
    UnknownContent(T),
}

impl<T: Debug> From<RequestError> for Error<T> {
    fn from(c: RequestError) -> Self {
        match c {
            RequestError::Json(json) => Error::Json(json),
            RequestError::Reqwest(reqwest) => Error::Reqwest(reqwest),
        }
    }
}

impl<T: Debug> From<ResponseError<T>> for Error<T> {
    fn from(r: ResponseError<T>) -> Self {
        match r {
            ResponseError::Reqwest(reqwest) => Error::Reqwest(reqwest),
            ResponseError::Json(json) => Error::Json(json),
            ResponseError::UnknownContent(unknown) => Error::UnknownContent(unknown),
        }
    }
}

/// Provides the `send` extension method to quickly call and read the result of a `cloudflare::Request`.
#[async_trait]
pub trait SendClientExt<E: Endpoint>: CallClientExt<reqwest::Client> {
    async fn send(self, client: &reqwest::Client) -> Result<E::Response, Error<reqwest::Response>>;
}

#[async_trait]
impl<E: Endpoint> SendClientExt<E> for request::Request<'_, E>
where
    E::Query: Serialize + Sync,
    E::Body: Content + Sync,
    E::Response: AsyncResponse,
{
    async fn send(self, client: &reqwest::Client) -> Result<E::Response, Error<reqwest::Response>> {
        Ok(self.call(client)?.send().await?.read::<E>().await?)
    }
}

/// Provides the `send` extension method to quickly call and read the result of a `cloudflare::Request`.
#[cfg(all(feature = "blocking", not(target_arch = "wasm32")))]
pub trait SendBlockingClientExt<E: Endpoint>: CallClientExt<blocking::Client> {
    fn send(self, client: &blocking::Client) -> Result<E::Response, Error<blocking::Response>>;
}

#[cfg(all(feature = "blocking", not(target_arch = "wasm32")))]
impl<E: Endpoint> SendBlockingClientExt<E> for request::Request<'_, E>
where
    E::Query: Serialize,
    E::Body: Content,
    E::Response: Response,
{
    fn send(self, client: &blocking::Client) -> Result<E::Response, Error<blocking::Response>> {
        Ok(self.call(client)?.send()?.read::<E>()?)
    }
}
