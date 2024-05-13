use std::future::Future;
use std::pin::Pin;

use crate::endpoints::{self, stats};
use crate::types;
use crate::utils::GetOwned;

pub struct Stats;
pub struct StatsPath;

crate::utils::impl_get_owned! {InvidiousStatEndpoint, String, String::from("/stats")}

impl stats::StatsEndpoints<'_, reqwest::Error> for Stats {
    type EndpointPath = InvidiousStatEndpoint;
    type OkStatsResponse = crate::types::api_info::InvidiousStats;
}

// Example implementation:
// async fn something() {
//     <Stats as stats::StatsEndpoints<'_, _>>::get_instance_stats(
//         &crate::endpoints::InstanceUrl::new("127.0.0.1".try_into().unwrap(), "/api/v1"),
//         |url| {
//             Box::pin(async move {
//                 let get_res = reqwest::get(url).await?;
//                 let bytes = get_res.bytes().await?;
//                 Ok(bytes.to_vec())
//             })
//         },
//     )
//     .await
//     .ok();
// }
