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
    async fn video_fetch_and_parse() {
        let res = REQWEST_INVIDIOUS
            .get_video_info(
                &get_instance(),
                VideoInfoParams {
                    video_id: "Wx92JT6IrKY",
                    region: IsoRegion::NP,
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn stats_fetch_and_parse() {
        let res = REQWEST_INVIDIOUS.get_instance_stats(&get_instance()).await;
        assert!(res.is_ok());
    }
}
