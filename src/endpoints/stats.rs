use super::error::Error;
use super::InstanceUrl;
use crate::types;

pub const STATS_API_PATH: &'static str = "/stats";
pub static STATS_ENDPOINT: super::CallableEndpoint = super::CallableEndpoint {
    endpoint_path: std::borrow::Cow::Borrowed(STATS_API_PATH),
    post_dynamic_path: None,
};
pub type OkCallbackResponse = types::api_info::InvidiousStats;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[repr(transparent)]
pub struct StatsEndpoint;
impl StatsEndpoint {
    pub async fn call_endpoint<CbError>(
        instance: &InstanceUrl,
        web_call_get: super::WebCallGet<CbError>,
    ) -> Result<OkCallbackResponse, Error<CbError>> {
        super::CallableEndpoint::call(
            &STATS_ENDPOINT,
            instance,
            Option::<&'static str>::None,
            Option::<&[(&'static str, Option<&'static str>)]>::None,
            web_call_get,
        )
        .await
    }
}
