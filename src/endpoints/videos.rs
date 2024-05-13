use super::error::Error;
use super::InstanceUrl;
use crate::types;

pub const VIDEO_INFO_PATH: &'static str = "/videos";
pub static VIDEO_INFO_ENDPOINT: super::CallableEndpoint = super::CallableEndpoint {
    endpoint_path: std::borrow::Cow::Borrowed(VIDEO_INFO_PATH),
    post_dynamic_path: None,
};
pub type VideoInfoResponse = types::video::VideoInfo;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[repr(transparent)]
pub struct VideoInfoEndpoint;
impl VideoInfoEndpoint {
    pub async fn call_endpoint<CbError>(
        instance: &InstanceUrl,
        video_id: impl AsRef<str>,
        web_call_get: super::WebCallGet<CbError>,
    ) -> Result<VideoInfoResponse, Error<CbError>> {
        super::CallableEndpoint::call(
            &VIDEO_INFO_ENDPOINT,
            instance,
            Some(video_id),
            Option::<&[(&'static str, Option<&'static str>)]>::None,
            web_call_get,
        )
        .await
    }
}
