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

#[cfg(test)]
mod tests {
    use super::InvidiousEndpoint;
    use super::*;
    use crate::endpoints::videos::VideoInfoParams;
    use crate::endpoints::InstanceUrl;
    use crate::types::region::IsoRegion;

    fn get_instance() -> InstanceUrl {
        InstanceUrl::new("https://vid.puffyan.us".parse().unwrap(), "/api/v1")
    }

    #[tokio::test]
    async fn fetch_and_parse() {
        let instance = get_instance();
        let query = VideoInfoParams {
            video_id: "Wx92JT6IrKY",
            region: IsoRegion::NP,
        };

        let res = REQWEST_INVIDIOUS.get_video_info(&instance, query).await;
        assert!(res.is_ok());
    }
}
