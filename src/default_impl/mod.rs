use crate::endpoints::stats;

// Get given url and return it's response as bytes
pub async fn get(url: url::Url) -> Result<Vec<u8>, reqwest::Error> {
    let get_res = reqwest::get(url).await?;
    let bytes = get_res.bytes().await?;
    Ok(bytes.to_vec())
}

pub struct Stats;
pub struct StatsPath;

crate::utils::impl_get_owned! {InvidiousStatEndpoint, String, String::from("/stats")}

impl stats::StatsEndpoints<'_, reqwest::Error> for Stats {
    type EndpointPath = InvidiousStatEndpoint;
    type OkStatsResponse = crate::types::api_info::InvidiousStats;
}
