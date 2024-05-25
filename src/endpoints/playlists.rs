use super::error::Error;
use super::InstanceUrl;
use super::{CallableEndpoint, WebCallGet};
use crate::types;
use std::borrow::Cow;

pub const PLAYLIST_INFO_PATH: &'static str = "/playlists";
pub static PLAYLIST_INFO_ENDPOINT: CallableEndpoint = CallableEndpoint {
    endpoint_path: Cow::Borrowed(PLAYLIST_INFO_PATH),
    post_dynamic_path: None,
};
pub type OkCallbackResponse = types::playlists::PlaylistInfo;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PlaylistInfoParams<'a> {
    pub playlist_id: &'a str,
    pub page: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[repr(transparent)]
pub struct PlaylistInfoEndpoint;
impl PlaylistInfoEndpoint {
    pub async fn call_endpoint<CbError>(
        instance: &InstanceUrl,
        params: PlaylistInfoParams<'_>,
        web_call_get: WebCallGet<CbError>,
    ) -> Result<OkCallbackResponse, Error<CbError>> {
        let page_as_query = [("page", Some(params.page.to_string()))];

        CallableEndpoint::call(
            &PLAYLIST_INFO_ENDPOINT,
            instance,
            Some(params.playlist_id),
            Some(page_as_query.as_ref()),
            web_call_get,
        )
        .await
    }
}

pub mod channel {
    use super::*;

    pub const CHANNEL_PLAYLISTS_PATH: &'static str = "/trending";
    pub const CHANNEL_PLAYLISTS_ENDPOINT: CallableEndpoint = CallableEndpoint {
        endpoint_path: Cow::Borrowed(CHANNEL_PLAYLISTS_PATH),
        post_dynamic_path: None,
    };

    pub type ChannelPlaylistsResponse = types::video::TrendingVideos;
    #[derive(Debug, Clone)]
    pub struct ChannelPlaylistsParams<'a> {
        pub channel_id: &'a str,
        pub sort_by: types::common::PlaylistSortingOption,
        pub continuation: String,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Copy)]
    #[repr(transparent)]
    pub struct ChannelPlaylistsEndpoint;
    impl ChannelPlaylistsEndpoint {
        pub async fn call_endpoint<CbError>(
            instance: &InstanceUrl,
            params: ChannelPlaylistsParams<'_>,
            web_call_get: WebCallGet<CbError>,
        ) -> Result<ChannelPlaylistsResponse, Error<CbError>> {
            let query = [
                ("sort_by", Some(params.sort_by.as_str())),
                ("continuation", Some(params.continuation.as_str())),
            ];

            CallableEndpoint::call(
                &CHANNEL_PLAYLISTS_ENDPOINT,
                instance,
                Some(params.channel_id),
                Some(query.as_ref()),
                web_call_get,
            )
            .await
        }
    }
}


