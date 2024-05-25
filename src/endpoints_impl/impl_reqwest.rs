use super::impl_template::*;

// Get given url and return it's response as bytes
pub async fn reqwest_get_bytes(url: url::Url) -> Result<Vec<u8>, reqwest::Error> {
    let get_res = reqwest::get(url).await?;
    let bytes = get_res.bytes().await?;
    Ok(bytes.to_vec())
}

pub struct ReqwestInvidious;
pub const REQWEST_INVIDIOUS: ReqwestInvidious = ReqwestInvidious;

impl InvidiousEndpoint for ReqwestInvidious {
    type CallbackError = reqwest::Error;
    const WEB_CALL_GET: CallbackFn<Self> = |url| Box::pin(reqwest_get_bytes(url));
}
