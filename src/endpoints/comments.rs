use super::error::Error;
use super::CallableEndpoint;
use super::InstanceUrl;
use super::WebCallGet;
use crate::types;
use std::borrow::Cow;

pub const COMMENT_INFO_PATH: &'static str = "/comments";
pub static COMMENT_INFO_ENDPOINT: CallableEndpoint = CallableEndpoint {
    endpoint_path: Cow::Borrowed(COMMENT_INFO_PATH),
    post_dynamic_path: None,
};
pub type OkCallbackResponse = types::common::CommentInfo;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommentParams<'a> {
    pub id: &'a str,
    pub continuation: String,
    pub sort_by: crate::types::common::CommentSorting,
    pub source: crate::types::common::CommentSource,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[repr(transparent)]
pub struct CommentInfoEndpoint;
impl CommentInfoEndpoint {
    pub async fn call_endpoint<CbError>(
        instance: &InstanceUrl,
        params: CommentParams<'_>,
        web_call_get: WebCallGet<CbError>,
    ) -> Result<OkCallbackResponse, Error<CbError>> {
        let comment_query = [
            ("sort_by", Some(params.sort_by.as_str())),
            ("source", Some(params.source.as_str())),
            ("continuation", Some(params.continuation.as_str())),
        ];

        super::CallableEndpoint::call(
            &COMMENT_INFO_ENDPOINT,
            instance,
            Some(params.id),
            Some(comment_query.as_ref()),
            web_call_get,
        )
        .await
    }
}

pub mod channel {
    pub use crate::endpoints::channels::comments::*;
}
