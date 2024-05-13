use crate::utils::{GetOwned, GetRef};
pub mod error;
pub mod stats;

pub struct InstanceUrl {
    server_url: url::Url,
    api_root: String,
    api_base_url: url::Url,
}

impl InstanceUrl {
    pub fn new(server_url: url::Url, api_root: impl Into<String>) -> Self {
        let api_root = api_root.into();
        let mut api_base_url = server_url.clone();
        api_base_url
            .path_segments_mut()
            .expect("InstanceUrl expected to be able to base")
            .extend(&[api_root.as_str()]);
        Self {
            server_url,
            api_root,
            api_base_url,
        }
    }

    fn get_api_base_url(&self) -> &url::Url {
        &self.api_base_url
    }
}

/// InstanceUrl with Reference
/// Get the endpoint path of any api object
fn get_endpoint_path(
    instance: &InstanceUrl,
    path: &str,
    queries: &[(&str, Option<&str>)],
) -> url::Url {
    let mut url = instance.get_api_base_url().clone();

    // add path
    url.path_segments_mut()
        .expect("[EndpointPath::get_endpoint_path] InstanceUrl expected to be able to base")
        .push(path);

    // add query
    for (key, val) in queries {
        match val {
            Some(val) => url.query_pairs_mut().append_pair(key, val),
            None => url.query_pairs_mut().append_key_only(key),
        };
    }

    url
}
