use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

macro_rules! expand_error {
    ($($(#[$attr:meta])+ $error:tt $(,)?)*) => {
        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error("encode: {0}")]
            Encode(#[from] $crate::request::EncodeError),
            #[error("decode: {0}")]
            Decode(#[from] $crate::request::DecodeError),
            #[error("reqwest: {0}")]
            Reqwest(#[from] ::reqwest::Error),

            #[error("{0}: {1}")]
            Undefined(::http::StatusCode, String),

            $(
                $(
                    #[$attr]
                )+
                $error,
            )*
        }

        impl Error {
            pub(crate) async fn from_response(resp: ::reqwest::Response) -> Self {
                match resp.status() {
                    code => Self::Undefined(code, resp.text().await.unwrap_or_default())
                }
            }
        }
    };
}

pub(crate) use expand_error;
