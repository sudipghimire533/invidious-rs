use super::error::Error;
use super::InstanceUrl;
use crate::types;

pub const VIDEO_INFO_PATH: &'static str = "/videos";
pub static VIDEO_INFO_ENDPOINT: super::CallableEndpoint = super::CallableEndpoint {
    endpoint_path: std::borrow::Cow::Borrowed(VIDEO_INFO_PATH),
    post_dynamic_path: None,
};
pub type VideoInfoResponse = types::video::VideoInfo;

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
        web_call_get: super::WebCallGet<CbError>,
    ) -> Result<VideoInfoResponse, Error<CbError>> {
        let region_as_query = [("region", Some(params.region.as_str()))];

        super::CallableEndpoint::call(
            &VIDEO_INFO_ENDPOINT,
            instance,
            Some(params.video_id),
            Some(region_as_query.as_ref()),
            web_call_get,
        )
        .await
    }
}
