use super::error::Error;
use super::InstanceUrl;
use super::{CallableEndpoint, WebCallGet};
use crate::types;
use std::borrow::Cow;

pub const VIDEO_INFO_PATH: &'static str = "/videos";
pub static VIDEO_INFO_ENDPOINT: CallableEndpoint = CallableEndpoint {
    endpoint_path: Cow::Borrowed(VIDEO_INFO_PATH),
    post_dynamic_path: None,
};
pub type OkCallbackResponse = types::video::VideoInfo;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VideoInfoParams<'a> {
    pub video_id: &'a str,
    pub region: types::region::IsoRegion,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[repr(transparent)]
pub struct VideoInfoEndpoint;
impl VideoInfoEndpoint {
    pub async fn call_endpoint<CbError>(
        instance: &InstanceUrl,
        params: VideoInfoParams<'_>,
        web_call_get: WebCallGet<CbError>,
    ) -> Result<OkCallbackResponse, Error<CbError>> {
        let region_as_query = [("region", Some(params.region.as_str()))];

        CallableEndpoint::call(
            &VIDEO_INFO_ENDPOINT,
            instance,
            Some(params.video_id),
            Some(region_as_query.as_ref()),
            web_call_get,
        )
        .await
    }
}

pub mod trending {
    use super::*;

    pub const TRENDING_VIDEO_PATH: &'static str = "/trending";
    pub const TRENDING_VIDEO_ENDPOINT: CallableEndpoint = CallableEndpoint {
        endpoint_path: Cow::Borrowed(TRENDING_VIDEO_PATH),
        post_dynamic_path: None,
    };

    pub type TrendingVideoResponse = types::video::TrendingVideos;
    #[derive(Debug, Clone)]
    pub struct TrendingVideoParams {
        pub region: types::region::IsoRegion,
        pub type_: types::common::ContentCategory,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Copy)]
    #[repr(transparent)]
    pub struct TrendingVideo;
    impl TrendingVideo {
        pub async fn call_endpoint<CbError>(
            instance: &InstanceUrl,
            trending_params: TrendingVideoParams,
            web_call_get: WebCallGet<CbError>,
        ) -> Result<TrendingVideoResponse, Error<CbError>> {
            let query = [
                ("region", Some(trending_params.region.as_str())),
                ("type", Some(trending_params.type_.as_str())),
            ];

            CallableEndpoint::call(
                &TRENDING_VIDEO_ENDPOINT,
                instance,
                Option::<&'static str>::None,
                Some(query.as_ref()),
                web_call_get,
            )
            .await
        }
    }
}

pub mod popular {
    use super::*;

    pub type PopularVideoResponse = types::video::PopularVideos;
    pub const POPULAR_VIDEO_PATH: &'static str = "/popular";
    pub const POPULAR_VIDEO_ENDPOINT: CallableEndpoint = CallableEndpoint {
        endpoint_path: Cow::Borrowed(POPULAR_VIDEO_PATH),
        post_dynamic_path: None,
    };

    #[derive(Debug, Clone, Eq, PartialEq, Copy)]
    #[repr(transparent)]
    pub struct PopularVideo;
    impl PopularVideo {
        pub async fn call_endpoint<CbError>(
            instance: &InstanceUrl,
            web_call_get: WebCallGet<CbError>,
        ) -> Result<PopularVideoResponse, Error<CbError>> {
            super::CallableEndpoint::call(
                &POPULAR_VIDEO_ENDPOINT,
                instance,
                Option::<&'static str>::None,
                Option::<&[(&'static str, Option<&'static str>)]>::None,
                web_call_get,
            )
            .await
        }
    }
}

pub mod shorts {
    use super::*;

    pub type ChannelShortsResponse = types::video::ChannelVideos;
    pub const CHANNEL_SHORTS_STR: &'static str = "/channel";
    pub const CHANNEL_SHORTS_ENDPOINT: CallableEndpoint = CallableEndpoint {
        endpoint_path: Cow::Borrowed(CHANNEL_SHORTS_STR),
        post_dynamic_path: Some(Cow::Borrowed("shorts")),
    };

    #[derive(Debug, Clone)]
    pub struct ChannelShortsParams<'a> {
        pub channel_id: &'a str,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Copy)]
    #[repr(transparent)]
    pub struct ChannelShortsEndpoint;
    impl ChannelShortsEndpoint {
        pub async fn call_endpoint<CbError>(
            instance: &InstanceUrl,
            channel_shorts_params: ChannelShortsParams<'_>,
            web_call_get: WebCallGet<CbError>,
        ) -> Result<ChannelShortsResponse, Error<CbError>> {
            super::CallableEndpoint::call(
                &CHANNEL_SHORTS_ENDPOINT,
                instance,
                Some(channel_shorts_params.channel_id),
                Option::<&[(&'static str, Option<&'static str>)]>::None,
                web_call_get,
            )
            .await
        }
    }
}

pub mod channel {
    use super::*;

    pub type ChannelVideosResponse = types::video::ChannelVideos;
    pub const CHANNEL_VIDEOS_STR: &'static str = "/channel";
    pub const CHANNEL_VIDEOS_ENDPOINT: CallableEndpoint = CallableEndpoint {
        endpoint_path: Cow::Borrowed(CHANNEL_VIDEOS_STR),
        post_dynamic_path: Some(Cow::Borrowed("videos")),
    };

    #[derive(Debug, Clone)]
    pub struct ChannelVideosParams<'a> {
        pub channel_id: &'a str,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Copy)]
    #[repr(transparent)]
    pub struct ChannelVideos;
    impl ChannelVideos {
        pub async fn call_endpoint<CbError>(
            instance: &InstanceUrl,
            channel_shorts_params: ChannelVideosParams<'_>,
            web_call_get: WebCallGet<CbError>,
        ) -> Result<ChannelVideosResponse, Error<CbError>> {
            super::CallableEndpoint::call(
                &CHANNEL_VIDEOS_ENDPOINT,
                instance,
                Some(channel_shorts_params.channel_id),
                Option::<&[(&'static str, Option<&'static str>)]>::None,
                web_call_get,
            )
            .await
        }
    }
}
