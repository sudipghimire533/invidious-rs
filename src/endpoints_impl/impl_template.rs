use std::{future::Future, pin::Pin};

use crate::endpoints::{self, InstanceUrl};

pub type CallbackErrorOf<T> = <T as InvidiousEndpoint>::CallbackError;
pub type EndpointErrorOf<T> = endpoints::error::Error<CallbackErrorOf<T>>;
pub type EndpointResultOf<T, Ok> = Result<Ok, EndpointErrorOf<T>>;
pub type CallbackFn<T> =
    fn(url::Url) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, CallbackErrorOf<T>>>>>;

pub trait InvidiousEndpoint {
    type CallbackError;
    const WEB_CALL_GET: CallbackFn<Self>;

    async fn get_instance_stats(
        &self,
        instance: &InstanceUrl,
    ) -> EndpointResultOf<Self, endpoints::stats::OkCallbackResponse> {
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
    ) -> EndpointResultOf<Self, endpoints::videos::OkCallbackResponse> {
        endpoints::videos::VideoInfoEndpoint::call_endpoint::<Self::CallbackError>(
            instance,
            query_params,
            Self::WEB_CALL_GET,
        )
        .await
    }

    async fn get_playlist_info(
        &self,
        instance: &InstanceUrl,
        query_params: endpoints::playlists::PlaylistInfoParams<'_>,
    ) -> EndpointResultOf<Self, endpoints::playlists::OkCallbackResponse>{
        endpoints::playlists::PlaylistInfoEndpoint::call_endpoint::<Self::CallbackError>(
            instance,
            query_params,
            Self::WEB_CALL_GET,
        )
        .await
    }

    async fn get_comment_info(
        &self,
        instance: &InstanceUrl,
        query_params: endpoints::comments::CommentParams<'_>,
    ) -> EndpointResultOf<Self, endpoints::comments::OkCallbackResponse> {
        endpoints::comments::CommentInfoEndpoint::call_endpoint::<Self::CallbackError>(
            instance,
            query_params,
            Self::WEB_CALL_GET,
        )
        .await
    }

    async fn get_channel_comment(
        &self,
        instance: &InstanceUrl,
        query_params: endpoints::channels::comments::ChannelCommentParams<'_>,
    ) -> EndpointResultOf<Self, endpoints::channels::comments::OkCallbackResponse> {
        endpoints::channels::comments::ChannelCommentEndpoint::call_endpoint::<Self::CallbackError>(
            instance,
            query_params,
            Self::WEB_CALL_GET,
        )
        .await
    }

    async fn get_channel_shorts(
        &self,
        instance: &InstanceUrl,
        query_params: endpoints::channels::shorts::ChannelShortsParams<'_>,
    ) -> EndpointResultOf<Self, endpoints::channels::shorts::ChannelShortsResponse> {
        endpoints::channels::shorts::ChannelShortsEndpoint::call_endpoint::<Self::CallbackError>(
            instance,
            query_params,
            Self::WEB_CALL_GET,
        )
        .await
    }

    async fn get_channel_playlists(
        &self,
        instance: &InstanceUrl,
        query_params: endpoints::playlists::channel::ChannelPlaylistsParams<'_>,
    ) -> EndpointResultOf<Self, endpoints::playlists::channel::OkCallbackResponse> {
        endpoints::playlists::channel::ChannelPlaylistsEndpoint::call_endpoint::<Self::CallbackError>(
            instance,
            query_params,
            Self::WEB_CALL_GET
        ).await
    }
}
