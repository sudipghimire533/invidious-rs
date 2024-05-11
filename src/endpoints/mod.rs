use crate::utils::GetRef;
pub mod error;
pub mod stats;

/// Information related to url of this instance
pub trait InstanceUrl {
    /// Get only the domain path of instance
    /// visiting this url directly will be the instance UI
    fn get_server_url(&self) -> &url::Url;

    /// give the base path under which all apis are stored
    fn get_api_root(&self) -> &str;

    /// Any api endpoints must be prefixed under this url and can be called
    /// Same as concatination of Self::get_server_url() + Self::get_api_root()
    fn get_api_base_url(&self) -> url::Url {
        let mut url = Self::get_server_url(self).clone();
        url.path_segments_mut()
            .expect("InstanceUrl expected to be able to base")
            .extend(&[Self::get_api_root(self)]);
        url
    }
}

/// InstanceUrl with Reference
/// Get the endpoint path of any api object
pub trait EndpointPath<'me>
where
    Self: 'me,
{
    type Path: GetRef<'me, String>;

    fn get_endpoint_path(instance: &impl InstanceUrl) -> url::Url {
        let mut url = instance.get_api_base_url();
        url.path_segments_mut()
            .expect("[EndpointPath::get_endpoint_path] InstanceUrl expected to be able to base")
            .push(Self::Path::get_ref().as_str());
        url
    }
}

/// Middleware Callback for GET method.
/// this is to allow web request to be called by the user itself
/// This allow user to use any http web client and make callbacks
/// specific to the server/proxy they use
pub trait CallBackGet<Res> {
    type Callback: Fn(&url::Url) -> Res;

    fn callback_get(callback: &Self::Callback, url: &url::Url) -> Res {
        callback(url)
    }
}
