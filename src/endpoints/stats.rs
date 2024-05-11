use super::error::Error;
use super::CallBackGet;
use super::EndpointPath;
use super::InstanceUrl;
use crate::types;
use serde::de::DeserializeOwned;

type ExpectedCallbackOkResponse<'a> = &'a [u8];
type ExpectedCallbackErrorResponse = types::common::SimpleError;
type ExpectedCallbackResponse<'a> =
    Result<ExpectedCallbackOkResponse<'a>, ExpectedCallbackErrorResponse>;

/// Get stats of this instance
/// https://docs.invidious.io/api/#get-apiv1stats
pub trait StatsEndpoints<'ep, 'res> {
    // possible: /stats
    type EndpointPath: EndpointPath<'ep>;
    // final output for this callback
    // possibly: types::InvidiousStats
    type OkStatsResponse: DeserializeOwned;
    type CallBack: CallBackGet<ExpectedCallbackResponse<'res>>;

    fn get_instance_stats(
        instance: &impl InstanceUrl,
        callback: &<Self::CallBack as CallBackGet<ExpectedCallbackResponse<'res>>>::Callback,
    ) -> Result<Self::OkStatsResponse, Error<ExpectedCallbackErrorResponse>> {
        let endpoint_path = Self::EndpointPath::get_endpoint_path(instance);
        let response = Self::CallBack::callback_get(callback, &endpoint_path)?;

        serde_json::from_slice(response)
            .map_err(|e| Error::DeserilizationError(Some(format!("{e:?}"))))
    }
}
