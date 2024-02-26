use bytes::Bytes;
use http::{Method, StatusCode};
use serde::Serialize;

use crate::{
    error::expand_error,
    model,
    request::{Auth, ChzzkResponse, Decode, DecodeError, Encode, EncodeError, Request},
    CHZZK_API_URL,
};

expand_error![];

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct GetLiveDetail<'a> {
    pub channel_id: &'a str,
}

impl<'a> GetLiveDetail<'a> {
    pub async fn send(&self, token: impl Into<Option<&Auth>>) -> Result<model::LiveDetail, Error> {
        let resp = self.encode_ref()?.send(token).await?;

        match resp.status() {
            StatusCode::OK => {
                let r = GetLiveDetail::decode(resp.bytes().await?)?;

                Ok(r)
            }
            _ => Err(Error::from_response(resp).await),
        }
    }
}

#[derive(Serialize)]
struct Path<'a> {
    streamer_id: &'a str,
}

impl<'a> Encode for GetLiveDetail<'a> {
    fn encode_ref(&self) -> Result<Request, EncodeError> {
        let GetLiveDetail {
            channel_id: streamer_id,
        } = *self;

        let path = serde_path::to_string(
            "/service/v2/channels/:streamer_id/live-detail",
            &Path { streamer_id },
        )?;

        Ok(Request {
            base_url: CHZZK_API_URL,
            method: Method::GET,
            path: path.into(),
            headers: None,
            body: None,
            query: None,
        })
    }
}

impl<'a> Decode for GetLiveDetail<'a> {
    type Output = model::LiveDetail;

    fn decode(bytes: Bytes) -> Result<Self::Output, DecodeError> {
        // println!("{}", String::from_utf8(bytes.to_vec()).unwrap());

        let deserialized: ChzzkResponse<model::sealed::LiveDetail> =
            serde_json::from_slice(&bytes)?;

        Ok(deserialized.content.try_into()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detail() {
        let live_detail = GetLiveDetail {
            channel_id: "475313e6c26639d5763628313b4c130e",
        }
        .send(None)
        .await
        .unwrap();

        println!("{:#?}", live_detail);
    }
}
