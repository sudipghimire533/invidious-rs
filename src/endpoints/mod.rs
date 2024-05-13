use std::{
    boxed::Box,
    convert::AsRef,
    future::{self, Future},
    pin::Pin,
};

use serde::de::DeserializeOwned;

use crate::utils::{self, GetOwned, GetRef};

use self::error::Error;
pub mod error;
pub mod stats;
pub mod videos;

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

pub type WebCallGet<CbError> =
    fn(url::Url) -> Pin<Box<dyn Future<Output = stats::ExpectedRes<CbError>>>>;

pub trait CallableEndpoint<CbError>
where
    Self: Sized,
{
    type DynamicPath: ToString;
    type PostDynamicPath: GetOwned<String>;
    type EndpointPath: GetOwned<String>;
    type OkCallbackResponse: DeserializeOwned;

    #[inline]
    async fn with_queries(
        instance: &InstanceUrl,
        queries: &[(impl AsRef<str>, Option<impl AsRef<str>>)],
        web_call_get: WebCallGet<CbError>,
    ) -> Result<Self::OkCallbackResponse, Error<CbError>> {
        Self::_call(instance, None, Some(queries), web_call_get).await
    }

    #[inline]
    async fn with_dynamic_path(
        instance: &InstanceUrl,
        dynamic_path: Self::DynamicPath,
        web_call_get: WebCallGet<CbError>,
    ) -> Result<Self::OkCallbackResponse, Error<CbError>> {
        Self::_call(
            instance,
            Some(dynamic_path),
            Option::<&[(&'static str, Option<&'static str>)]>::None,
            web_call_get,
        )
        .await
    }

    #[inline]
    async fn with_dynamic_path_and_queries(
        instance: &InstanceUrl,
        dynamic_path: Self::DynamicPath,
        queries: &[(impl AsRef<str>, Option<impl AsRef<str>>)],
        web_call_get: WebCallGet<CbError>,
    ) -> Result<Self::OkCallbackResponse, Error<CbError>> {
        Self::_call(instance, Some(dynamic_path), Some(queries), web_call_get).await
    }

    async fn _call(
        instance: &InstanceUrl,
        dynamic_path: Option<Self::DynamicPath>,
        queries: Option<&[(impl AsRef<str>, Option<impl AsRef<str>>)]>,
        web_call_get: WebCallGet<CbError>,
    ) -> Result<Self::OkCallbackResponse, Error<CbError>> {
        let mut paths = vec![Self::EndpointPath::get_owned()];
        if let Some(dynamic_path) = dynamic_path {
            paths.push(dynamic_path.to_string());
            paths.push(Self::PostDynamicPath::get_owned());
        }

        let endpoint_path = get_endpoint_path(instance, &paths, queries.unwrap_or_else(|| &[]));
        let response = web_call_get(endpoint_path).await?;

        let result: Result<
            utils::UntaggeBinary<Self::OkCallbackResponse, crate::types::common::SimpleError>,
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
}
