use std::{borrow::Cow, fmt::Debug};

use bytes::Bytes;
use cookie::Cookie;
use http::{header, HeaderMap, Method};
use mime::Mime;
use reqwest::Response;
use serde::{Deserialize, Serialize};

pub struct Request {
    pub(crate) base_url: &'static str,
    pub(crate) method: Method,
    pub(crate) path: Cow<'static, str>,
    pub(crate) headers: Option<HeaderMap>,
    pub(crate) body: Option<RequestBody>,
    pub(crate) query: Option<Cow<'static, str>>,
}

pub struct RequestBody {
    pub(crate) content_type: Mime,
    pub(crate) buf: Bytes,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Auth {
    pub nid_ses: String,
    pub nid_aut: String,
    pub nid_jkl: String,
}

pub trait IntoBody {
    fn into_body(self) -> Result<RequestBody, EncodeError>;
}

impl Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.method)
            .field("base_url", &self.base_url)
            .field("path", &self.path)
            .field("query", &self.query)
            .field("headers", &self.headers)
            .finish()
    }
}

impl Request {
    fn build(self, token: Option<&Auth>) -> reqwest::RequestBuilder {
        tracing::debug!("{self:#?}");

        let Request {
            base_url,
            method,
            headers,
            path,
            body,
            query,
        } = self;

        let mut url = String::from(base_url) + &path;

        if let Some(query) = query {
            if !query.is_empty() {
                url += "?";
                url += &query;
            }
        }

        let mut request = reqwest::Client::new().request(method.clone(), url);

        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        if let Some(Auth {
            nid_ses,
            nid_aut,
            nid_jkl,
        }) = token
        {
            let cookie = Cookie::from_iter([
                ("NID_SES", &**nid_ses),
                ("NID_AUT", &**nid_aut),
                ("NID_JKL", &**nid_jkl),
            ]);

            request = request.header(header::COOKIE, cookie.into_str());
        }

        if let Some(RequestBody { content_type, buf }) = body {
            request = request
                .header(header::CONTENT_TYPE, content_type.as_ref())
                .body(buf);
        }

        request
    }

    pub(crate) async fn send(self, token: impl Into<Option<&Auth>>) -> reqwest::Result<Response> {
        self.build(token.into()).send().await
    }
}

#[derive(Debug, Deserialize)]
pub struct ChzzkResponse<T: Debug> {
    pub code: u16,
    pub message: Option<String>,
    pub content: T,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum EncodeError {
    #[error("serialize path: {0}")]
    SerializePath(#[from] serde_path::Error),

    #[error("serialize query: {0}")]
    SerializeQuery(#[from] serde_qs::Error),

    #[error("serialize json: {0}")]
    SerializeJson(#[from] serde_json::Error),
}

pub trait Encode: Sized {
    fn encode_ref(&self) -> Result<Request, EncodeError>;

    fn encode(self) -> Result<Request, EncodeError> {
        self.encode_ref()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("deserialize query: {0}")]
    DeserializeQuery(#[from] serde_qs::Error),
    #[error("deserialize json: {0}")]
    DeserializeJson(#[from] serde_json::Error),
}

pub trait Decode: Sized {
    type Output;

    fn decode(bytes: Bytes) -> Result<Self::Output, DecodeError>;
}
