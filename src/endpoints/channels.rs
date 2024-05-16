use super::error::Error;
use super::CallableEndpoint;
use super::InstanceUrl;
use super::WebCallGet;
use crate::types;
use std::borrow::Cow;

pub mod comments {
    use super::*;

    pub const CHANNEL_COMMENT_PATH: &'static str = "/channels/comments/";
    pub static CHANNEL_COMMENT_ENDPOINT: CallableEndpoint = CallableEndpoint {
        endpoint_path: Cow::Borrowed(CHANNEL_COMMENT_PATH),
        post_dynamic_path: None,
    };
    pub type OkCallbackResponse = types::common::ChannelComments;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ChannelCommentParams<'a> {
        pub id: &'a str,
        pub continuation: String,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Copy)]
    #[repr(transparent)]
    pub struct ChannelCommentEndpoint;
    impl ChannelCommentEndpoint {
        pub async fn call_endpoint<CbError>(
            instance: &InstanceUrl,
            params: ChannelCommentParams<'_>,
            web_call_get: WebCallGet<CbError>,
        ) -> Result<OkCallbackResponse, Error<CbError>> {
            let query = [("continuation", Some(params.continuation))];

            super::CallableEndpoint::call(
                &CHANNEL_COMMENT_ENDPOINT,
                instance,
                Some(params.id),
                Some(&query),
                web_call_get,
            )
            .await
        }
    }
}
