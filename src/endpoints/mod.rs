use std::{
    borrow::Cow,
    boxed::Box,
    convert::AsRef,
    future::{self, Future},
    marker::PhantomData,
    pin::Pin,
};

use serde::de::DeserializeOwned;

use crate::utils::{self, GetOwned, GetRef};

use self::error::Error;
pub mod comments;
pub mod error;
pub mod stats;
pub mod videos;

#[derive(Debug, Clone, Eq, PartialEq)]
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

pub type WebCallGet<CbError> =
    fn(url::Url) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, CbError>>>>;

pub struct CallableEndpoint {
    endpoint_path: Cow<'static, str>,
    post_dynamic_path: Option<Cow<'static, str>>,
}

impl CallableEndpoint {
    async fn call<OkCallbackResponse: DeserializeOwned + std::fmt::Debug, CbError>(
        &self,
        instance: &InstanceUrl,
        dynamic_path: Option<impl AsRef<str>>,
        queries: Option<&[(impl AsRef<str>, Option<impl AsRef<str>>)]>,
        web_call_get: WebCallGet<CbError>,
    ) -> Result<OkCallbackResponse, Error<CbError>> {
        let mut paths = vec![self.endpoint_path.as_ref()];
        if let Some(dynamic_path) = dynamic_path.as_ref() {
            paths.push(dynamic_path.as_ref());
            if let Some(post_dynamic_path) = self.post_dynamic_path.as_ref() {
                paths.push(post_dynamic_path.as_ref());
            }
        }

        let endpoint_path =
            Self::construct_full_path(instance, &paths, queries.unwrap_or_else(|| &[]));
        let response = web_call_get(endpoint_path).await?;

        let result: Result<
            utils::UntaggeBinary<OkCallbackResponse, crate::types::common::SimpleError>,
            _,
        > = serde_json::from_slice(response.as_slice());
        match result {
            Ok(utils::UntaggeBinary::Primary(stats)) => Ok(stats),
            Ok(utils::UntaggeBinary::Secondary(simple_error)) => {
                Err(Error::SimpleError(simple_error))
            }
            Err(de_err) => Err(Error::DeserilizationError(Some(format!("{de_err:?}")))),
        }
    }

    /// InstanceUrl with Reference
    /// Get the endpoint path of any api object
    fn construct_full_path(
        instance: &InstanceUrl,
        paths: &[impl AsRef<str>],
        queries: &[(impl AsRef<str>, Option<impl AsRef<str>>)],
    ) -> url::Url {
        let mut url = instance.get_api_base_url().clone();

        // add path
        url.path_segments_mut()
            .expect("[EndpointPath::get_endpoint_path] InstanceUrl expected to be able to base")
            .extend(paths);

        // add query
        for (key, val) in queries {
            match val {
                Some(val) => url
                    .query_pairs_mut()
                    .append_pair(key.as_ref(), val.as_ref()),
                None => url.query_pairs_mut().append_key_only(key.as_ref()),
            };
        }

        url
    }
}
