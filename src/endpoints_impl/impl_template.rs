#[macro_export]
macro_rules! implement_all_api {
($this_crate: path, $target: ident, $cbe: ty, $get_callback: expr) => {


use $this_crate::{
    endpoints::{self, InstanceUrl},
    types,
};

pub type EndpointError = endpoints::error::Error<reqwest::Error>;
pub type EndpointResult<Ok> = Result<Ok, EndpointError>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct $target;

impl $target {
    pub async fn get_instance_stats(
        instance: &InstanceUrl,
    ) -> EndpointResult<types::api_info::InvidiousStats> {
        endpoints::stats::StatsEndpoint::call_endpoint::<$cbe>(instance, |url| {
            Box::pin($get_callback(url))
        })
        .await
    }

    pub async fn get_video_info(
        instance: &InstanceUrl,
        query_params: endpoints::videos::VideoParams<'_>,
    ) -> EndpointResult<types::video::VideoInfo> {
        endpoints::videos::VideoInfoEndpoint::call_endpoint::<$cbe>(instance, query_params, |url| {
            Box::pin($get_callback(url))
        })
        .await
    }
}

}
}

#[allow(unused_imports)]
pub use implement_all_api;
