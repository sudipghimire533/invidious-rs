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
