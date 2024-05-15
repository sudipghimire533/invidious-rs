use std::{future::Future, pin::Pin};

use crate::{
    endpoints::{self, InstanceUrl},
    types,
};

pub(super) type CallbackErrorOf<T> = <T as InvidiousEndpoint>::CallbackError;
pub(super) type EndpointErrorOf<T> = endpoints::error::Error<CallbackErrorOf<T>>;
pub(super) type EndpointResultOf<T, Ok> = Result<Ok, EndpointErrorOf<T>>;
pub(super) type CallbackFn<T> =
    fn(url::Url) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, CallbackErrorOf<T>>>>>;

pub trait InvidiousEndpoint {
    type CallbackError;
    const WEB_CALL_GET: CallbackFn<Self>;

    async fn get_instance_stats(
        &self,
        instance: &InstanceUrl,
    ) -> EndpointResultOf<Self, types::api_info::InvidiousStats> {
        endpoints::stats::StatsEndpoint::call_endpoint::<Self::CallbackError>(
            instance,
            Self::WEB_CALL_GET,
        )
        .await
    }

    async fn get_video_info(
        &self,
        instance: &InstanceUrl,
        query_params: endpoints::videos::VideoInfoParams<'_>,
    ) -> EndpointResultOf<Self, types::video::VideoInfo> {
        endpoints::videos::VideoInfoEndpoint::call_endpoint::<Self::CallbackError>(
            instance,
            query_params,
            Self::WEB_CALL_GET,
        )
        .await
    }

    async fn get_comment_info(
        instance: &InstanceUrl,
        query_params: endpoints::comments::CommentParams<'_>,
    ) -> EndpointResultOf<Self, types::common::CommentInfo> {
        endpoints::comments::CommentInfoEndpoint::call_endpoint::<Self::CallbackError>(
            instance,
            query_params,
            Self::WEB_CALL_GET,
        )
        .await
    }
}
