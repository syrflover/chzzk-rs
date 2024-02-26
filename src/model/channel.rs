use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonalData {
    #[serde(rename = "privateUserBlock")]
    pub private_user_block: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialChannel {
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    #[serde(rename = "channelImageUrl")]
    pub channel_image_url: Option<String>,
    #[serde(rename = "verifiedMark")]
    pub verified_mark: bool,
    #[serde(rename = "userAdultStatus")]
    pub user_adult_status: Option<String>,
    #[serde(rename = "personalData")]
    pub personal_data: Option<PersonalData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Channel {
    #[serde(flatten)]
    pub inner: PartialChannel,

    #[serde(rename = "channelDescription")]
    pub channel_description: String,
    #[serde(rename = "followerCount")]
    pub follower_count: u64,
    #[serde(rename = "openLive")]
    pub open_live: bool,
}
