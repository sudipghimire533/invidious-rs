use super::error::Error;
use super::CallableEndpoint;
use super::InstanceUrl;
use super::WebCallGet;
use crate::types;
use std::borrow::Cow;

pub const SEARCH_API_PATH: &'static str = "/search";
pub static SEARCH_ENDPOINT: CallableEndpoint = CallableEndpoint {
    endpoint_path: Cow::Borrowed(SEARCH_API_PATH),
    post_dynamic_path: None,
};
pub type OkCallbackResponse = types::common::SearchResult;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SearchParams {
    pub q: String,
    pub page: i32,
    pub sort_by: types::common::SortingOption,
    pub duration: types::common::QueryResultDuration,
    pub date: types::common::QueryResultDate,
    pub search_type: types::common::SearchResultType,
    pub features: Vec<types::common::QueryResultFeatures>,
    pub region: types::region::IsoRegion,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[repr(transparent)]
pub struct StatsEndpoint;
impl StatsEndpoint {
    pub async fn call_endpoint<CbError>(
        instance: &InstanceUrl,
        params: SearchParams,
        web_call_get: WebCallGet<CbError>,
    ) -> Result<OkCallbackResponse, Error<CbError>> {
        let params_features = params
            .features
            .iter()
            .map(|a| a.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let params_page = params.page.to_string();
        let query = [
            ("q", Some(params.q.as_str())),
            ("page", Some(params_page.as_str())),
            ("sort_by", Some(params.sort_by.as_str())),
            ("date", Some(params.date.as_str())),
            ("duration", Some(params.duration.as_str())),
            ("search_type", Some(params.search_type.as_str())),
            ("region", Some(params.region.as_str())),
            if !params.features.is_empty() {
                ("features", Some(params_features.as_str()))
            } else {
                ("", None)
            },
        ];

        super::CallableEndpoint::call(
            &SEARCH_ENDPOINT,
            instance,
            Option::<&'static str>::None,
            Some(query.as_slice()),
            web_call_get,
        )
        .await
    }
}
