use super::EndpointPath;
use super::InstanceUrl;
use serde::Serialize;
use url::Url;

/// Get stats of this instance
/// https://docs.invidious.io/api/#get-apiv1stats
pub trait StatsEndpoints<'ep> {
    type EndpointPath: EndpointPath<'ep>;
    type OkStatsResponse: Serialize;

    fn get_instance_stats(instance: &impl InstanceUrl) -> Result<Self::OkStatsResponse, ()> {
        let endpoint_path = Self::EndpointPath::get_endpoint_path(instance);
        Err(())
    }
}
