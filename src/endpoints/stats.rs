use std::future::Future;
use std::pin::Pin;

use super::error::Error;
use super::InstanceUrl;
use crate::types;
use crate::types::common::SimpleError;
use crate::utils;
use crate::utils::GetOwned;
use serde::de::DeserializeOwned;

pub type ExpectedOk = Vec<u8>;
pub type ExpectedRes<Cbe> = Result<ExpectedOk, Error<Cbe>>;

/// Get stats of this instance
/// https://docs.invidious.io/api/#get-apiv1stats
pub trait StatsEndpoints<'ep, CbError> {
    // possible: /stats
    type EndpointPath: GetOwned<String>;
    // final output for this callback
    // possibly: types::InvidiousStats
    type OkStatsResponse: DeserializeOwned;

    async fn get_instance_stats(
        instance: &InstanceUrl,
        web_call_get: fn(url::Url) -> Pin<Box<dyn Future<Output = ExpectedRes<CbError>>>>,
    ) -> Result<Self::OkStatsResponse, Error<CbError>> {
        let endpoint_path =
            super::get_endpoint_path(instance, Self::EndpointPath::get_owned().as_str(), &[]);
        let response = web_call_get(endpoint_path).await?;

        let result: Result<utils::UntaggeBinary<Self::OkStatsResponse, SimpleError>, _> =
            serde_json::from_slice(response.as_slice());
        match result {
            Ok(utils::UntaggeBinary::Primary(stats)) => Ok(stats),
            Ok(utils::UntaggeBinary::Secondary(simple_error)) => {
                Err(Error::SimpleError(simple_error))
            }
            Err(de_err) => Err(Error::DeserilizationError(Some(format!("{de_err:?}")))),
        }
    }
}
