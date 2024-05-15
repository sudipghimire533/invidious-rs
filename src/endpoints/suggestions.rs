use super::error::Error;
use super::CallableEndpoint;
use super::InstanceUrl;
use super::WebCallGet;
use crate::types;
use std::borrow::Cow;

pub const SEARCH_SUGGESTION_PATH: &'static str = "/search/suggestion";
pub static SEARCH_SUGGESTION_ENDPOINT: CallableEndpoint = CallableEndpoint {
    endpoint_path: Cow::Borrowed(SEARCH_SUGGESTION_PATH),
    post_dynamic_path: None,
};
pub type SearchSuggestionResponse = types::common::SearchSuggestion;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SearchSuggestionParams<'a> {
    pub q: &'a str,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[repr(transparent)]
pub struct CommentInfoEndpoint;
impl CommentInfoEndpoint {
    pub async fn call_endpoint<CbError>(
        instance: &InstanceUrl,
        params: SearchSuggestionParams<'_>,
        web_call_get: WebCallGet<CbError>,
    ) -> Result<SearchSuggestionResponse, Error<CbError>> {
        let comment_query = [("q", Some(params.q))];

        CallableEndpoint::call(
            &SEARCH_SUGGESTION_ENDPOINT,
            instance,
            Option::<&'static str>::None,
            Some(comment_query.as_ref()),
            web_call_get,
        )
        .await
    }
}
