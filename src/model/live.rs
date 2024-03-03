use serde::{Deserialize, Serialize};

use super::channel::PartialChannel;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LiveStatusType {
    Open,
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CategoryType {
    Game,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LivePollingStatusType {
    Started,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayableStatusType {
    Playable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserAdultStatusType {
    Adult,
    NotLoginUser,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CdnInfo {
    #[serde(rename = "cdnType")]
    pub cdn_type: String, // TODO: to enum?

    #[serde(rename = "zeroRating")]
    pub zero_rating: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivePlaybackMeta {
    #[serde(rename = "videoId")]
    pub video_id: String,

    #[serde(rename = "streamSeq")]
    pub stream_seq: u64,
    // #[serde(rename = "liveId")]
    // pub live_id: String,
    #[serde(rename = "paidLive")]
    pub paid_live: bool,

    #[serde(rename = "cdnInfo")]
    pub cdn_info: CdnInfo,

    #[serde(rename = "cmcdEnabled")]
    pub cmcd_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivePlaybackServiceMeta {
    #[serde(rename = "contentType")]
    pub content_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LivePlaybackStatusType {
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "ENDED")]
    Ended,
    #[serde(rename = "STOPPED")]
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivePlaybackStatus {
    /// date
    pub start: String,
    /// date
    pub open: String,

    #[serde(rename = "timeMachine")]
    pub time_machine: bool,

    pub status: LivePlaybackStatusType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivePlaybackApi {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodingTrack {
    /// if audioOnly = true encodingTrackId: "alow.stream";
    #[serde(rename = "encodingTrackId")]
    pub encoding_track_id: String,

    #[serde(rename = "audioBitRate")]
    pub audio_bit_rate: usize,

    #[serde(rename = "audioSamplingRate")]
    pub audio_sampling_rate: usize,

    #[serde(rename = "audioChannel")]
    pub audio_channel: u8,

    #[serde(rename = "avoidReencoding")]
    pub avoid_reencoding: bool,

    #[serde(rename = "audioOnly")]
    pub audio_only: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoEncodingTrack {
    #[serde(flatten)]
    pub inherit: EncodingTrack,

    #[serde(rename = "videoProfile")]
    pub video_profile: String,

    #[serde(rename = "audioProfile")]
    pub audio_profile: String,

    #[serde(rename = "videoCodec")]
    pub video_codec: String,

    #[serde(rename = "videoBitRate")]
    pub video_bit_rate: usize,

    #[serde(rename = "videoFrameRate")]
    pub video_frame_rate: f32,

    #[serde(rename = "videoWidth")]
    pub video_width: usize,

    #[serde(rename = "videoHeight")]
    pub video_height: usize,

    #[serde(rename = "videoDynamicRange")]
    pub video_dynamic_range: String,
}

impl Eq for VideoEncodingTrack {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioEncodingTrack {
    pub path: String,
    #[serde(rename = "audioCodec")]
    pub audio_codec: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivePlaybackMedia {
    #[serde(rename = "mediaId")]
    pub media_id: String,
    pub protocol: String,
    pub path: String,

    #[serde(rename = "encodingTrack")]
    pub encoding_track: Vec<EncodingTrack>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivePlayback {
    pub meta: LivePlaybackMeta,
    #[serde(rename = "serviceMeta")]
    pub service_meta: LivePlaybackServiceMeta,
    pub live: LivePlaybackStatus,
    pub api: Vec<LivePlaybackApi>,
    pub media: Vec<LivePlaybackMedia>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LivePollingStatus {
    pub status: LivePollingStatusType,

    #[serde(rename = "isPublishing")]
    pub is_publishing: bool,

    #[serde(rename = "playableStatus")]
    pub playable_status: String,

    #[serde(rename = "trafficThrottling")]
    pub traffic_throttling: i32,

    #[serde(rename = "callPeriodMilliSecond")]
    pub call_period_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveStatus {
    pub live_title: String,
    pub status: LiveStatusType,
    pub concurrent_user_count: u64,
    pub accumulate_count: u64,
    pub paid_promotion: bool,
    pub adult: bool,
    pub chat_channel_id: Option<String>,
    pub category_type: Option<String>,
    pub live_category: Option<String>,
    pub live_category_value: Option<String>,
    pub live_polling_status: LivePollingStatus,
    pub user_adult_status: Option<UserAdultStatusType>,
    pub chat_active: bool,
    pub chat_available_group: String,
    pub chat_available_condition: String,
    pub min_follower_minute: u64,
}

impl TryFrom<sealed::LiveStatus> for LiveStatus {
    type Error = serde_json::Error;

    fn try_from(
        sealed::LiveStatus {
            live_title,
            status,
            concurrent_user_count,
            accumulate_count,
            paid_promotion,
            adult,
            chat_channel_id,
            category_type,
            live_category,
            live_category_value,
            live_polling_status_json,
            user_adult_status,
            chat_active,
            chat_available_group,
            chat_available_condition,
            min_follower_minute,
        }: sealed::LiveStatus,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            live_title,
            status,
            concurrent_user_count,
            accumulate_count,
            paid_promotion,
            adult,
            chat_channel_id,
            category_type,
            live_category,
            live_category_value,
            live_polling_status: serde_json::from_str(&live_polling_status_json)?,
            user_adult_status,
            chat_active,
            chat_available_group,
            chat_available_condition,
            min_follower_minute,
        })
    }
}

impl From<LiveDetail> for LiveStatus {
    fn from(
        LiveDetail {
            inherit,
            status,
            close_date: _,
            chat_active,
            chat_available_group,
            paid_promotion,
            chat_available_condition,
            min_follower_minute,
            live_polling_status,
            user_adult_status,
        }: LiveDetail,
    ) -> Self {
        Self {
            live_title: inherit.live_title,
            status,
            concurrent_user_count: inherit.concurrent_user_count,
            accumulate_count: inherit.accumulate_count,
            paid_promotion,
            adult: inherit.adult,
            chat_channel_id: inherit.chat_channel_id,
            category_type: inherit.category_type,
            live_category: inherit.live_category,
            live_category_value: inherit.live_category_value,
            live_polling_status,
            user_adult_status,
            chat_active,
            chat_available_group,
            chat_available_condition,
            min_follower_minute,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Live {
    pub live_title: String,
    pub live_image_url: Option<String>,
    pub default_thumbnail_image_url: Option<String>,
    pub concurrent_user_count: u64,
    pub accumulate_count: u64,
    pub open_date: String,
    pub live_id: u64,
    pub adult: bool,
    pub chat_channel_id: Option<String>,
    pub category_type: Option<String>,
    pub live_category: Option<String>,
    pub live_category_value: Option<String>,
    // pub channel_id: String,
    pub live_playback: Option<LivePlayback>,
    pub channel: PartialChannel,
}

impl TryFrom<sealed::Live> for Live {
    type Error = serde_json::Error;

    fn try_from(
        sealed::Live {
            live_title,
            live_image_url,
            default_thumbnail_image_url,
            concurrent_user_count,
            accumulate_count,
            open_date,
            live_id,
            adult,
            chat_channel_id,
            category_type,
            live_category,
            live_category_value,
            // channel_id,
            live_playback_json,
            channel,
        }: sealed::Live,
    ) -> Result<Self, Self::Error> {
        let live_playback = match live_playback_json {
            Some(x) => Some(serde_json::from_str(&x)?),
            None => None,
        };

        Ok(Self {
            live_title,
            live_image_url,
            default_thumbnail_image_url,
            concurrent_user_count,
            accumulate_count,
            open_date,
            live_id,
            adult,
            chat_channel_id,
            category_type,
            live_category,
            live_category_value,
            // channel_id,
            live_playback,
            channel,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveDetail {
    #[serde(flatten)]
    pub inherit: Live,
    pub status: LiveStatusType,
    pub close_date: Option<String>,
    pub chat_active: bool,
    pub chat_available_group: String,
    pub paid_promotion: bool,
    pub chat_available_condition: String,
    pub min_follower_minute: u64,
    pub live_polling_status: LivePollingStatus,
    pub user_adult_status: Option<UserAdultStatusType>,
}

impl TryFrom<sealed::LiveDetail> for LiveDetail {
    type Error = serde_json::Error;

    fn try_from(
        sealed::LiveDetail {
            inherit,
            status,
            close_date,
            chat_active,
            chat_available_group,
            paid_promotion,
            chat_available_condition,
            min_follower_minute,
            live_polling_status_json,
            user_adult_status,
        }: sealed::LiveDetail,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            inherit: inherit.try_into()?,
            status,
            close_date,
            chat_active,
            chat_available_group,
            paid_promotion,
            chat_available_condition,
            min_follower_minute,
            live_polling_status: serde_json::from_str(&live_polling_status_json)?,
            user_adult_status,
        })
    }
}

pub(crate) mod sealed {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct Live {
        #[serde(rename = "liveTitle")]
        pub(super) live_title: String,

        #[serde(rename = "liveImageUrl")]
        pub(super) live_image_url: Option<String>,

        #[serde(rename = "defaultThumbnailImageUrl")]
        pub(super) default_thumbnail_image_url: Option<String>,

        #[serde(rename = "concurrentUserCount")]
        pub(super) concurrent_user_count: u64,

        #[serde(rename = "accumulateCount")]
        pub(super) accumulate_count: u64,

        #[serde(rename = "openDate")]
        pub(super) open_date: String,

        #[serde(rename = "liveId")]
        pub(super) live_id: u64,

        #[serde(rename = "adult")]
        pub(super) adult: bool,

        #[serde(rename = "chatChannelId")]
        pub(super) chat_channel_id: Option<String>,

        #[serde(rename = "categoryType")]
        pub(super) category_type: Option<String>,

        #[serde(rename = "liveCategory")]
        pub(super) live_category: Option<String>,

        #[serde(rename = "liveCategoryValue")]
        pub(super) live_category_value: Option<String>,

        // #[serde(rename = "channelId")]
        // pub(super) channel_id: String,
        #[serde(rename = "livePlaybackJson")]
        pub(super) live_playback_json: Option<String>,

        #[serde(rename = "channel")]
        pub(super) channel: PartialChannel,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct LiveStatus {
        #[serde(rename = "liveTitle")]
        pub(super) live_title: String,

        #[serde(rename = "status")]
        pub(super) status: LiveStatusType,

        #[serde(rename = "concurrentUserCount")]
        pub(super) concurrent_user_count: u64,

        #[serde(rename = "accumulateCount")]
        pub(super) accumulate_count: u64,

        #[serde(rename = "paidPromotion")]
        pub(super) paid_promotion: bool,

        #[serde(rename = "adult")]
        pub(super) adult: bool,

        #[serde(rename = "chatChannelId")]
        pub(super) chat_channel_id: Option<String>,

        #[serde(rename = "categoryType")]
        pub(super) category_type: Option<String>,

        #[serde(rename = "liveCategory")]
        pub(super) live_category: Option<String>,

        #[serde(rename = "liveCategoryValue")]
        pub(super) live_category_value: Option<String>,

        #[serde(rename = "livePollingStatusJson")]
        pub(super) live_polling_status_json: String,

        #[serde(rename = "userAdultStatus")]
        pub(super) user_adult_status: Option<UserAdultStatusType>,

        #[serde(rename = "chatActive")]
        pub(super) chat_active: bool,

        #[serde(rename = "chatAvailableGroup")]
        pub(super) chat_available_group: String,

        #[serde(rename = "chatAvailableCondition")]
        pub(super) chat_available_condition: String,

        #[serde(rename = "minFollowerMinute")]
        pub(super) min_follower_minute: u64,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct LiveDetail {
        #[serde(flatten)]
        pub(super) inherit: Live,

        #[serde(rename = "status")]
        pub(super) status: LiveStatusType,

        #[serde(rename = "closeDate")]
        pub(super) close_date: Option<String>,

        #[serde(rename = "chatActive")]
        pub(super) chat_active: bool,

        #[serde(rename = "chatAvailableGroup")]
        pub(super) chat_available_group: String,

        #[serde(rename = "paidPromotion")]
        pub(super) paid_promotion: bool,

        #[serde(rename = "chatAvailableCondition")]
        pub(super) chat_available_condition: String,

        #[serde(rename = "minFollowerMinute")]
        pub(super) min_follower_minute: u64,

        #[serde(rename = "livePollingStatusJson")]
        pub(super) live_polling_status_json: String,

        #[serde(rename = "userAdultStatus")]
        pub(super) user_adult_status: Option<UserAdultStatusType>,
    }
}
