use crate::{
    endpoints::{self, InstanceUrl},
    types,
};

pub type EndpointError = crate::endpoints::error::Error<reqwest::Error>;
pub type EndpointResult<Ok> = Result<Ok, EndpointError>;

// Get given url and return it's response as bytes
pub async fn reqwest_get_bytes(url: url::Url) -> Result<Vec<u8>, reqwest::Error> {
    let get_res = reqwest::get(url).await?;
    let bytes = get_res.bytes().await?;
    Ok(bytes.to_vec())
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InvidiousReqwest;

impl InvidiousReqwest {
    pub async fn get_instance_stats(
        instance: &InstanceUrl,
    ) -> EndpointResult<types::api_info::InvidiousStats> {
        endpoints::stats::StatsEndpoint::call_endpoint(instance, |url| {
            Box::pin(reqwest_get_bytes(url))
        })
        .await
    }

    pub async fn get_video_info(
        instance: &InstanceUrl,
        video_id: impl AsRef<str>,
    ) -> EndpointResult<types::video::VideoInfo> {
        endpoints::videos::VideoInfoEndpoint::call_endpoint(instance, video_id, |url| {
            Box::pin(reqwest_get_bytes(url))
        })
        .await
    }
}
