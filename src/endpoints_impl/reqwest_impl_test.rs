use super::impl_reqwest::*;
use super::impl_template::*;

use crate::endpoints::channels::ChannelInfoParams;
use crate::endpoints::playlists::PlaylistInfoParams;
use crate::endpoints::videos::VideoInfoParams;
use crate::endpoints::InstanceUrl;
use crate::types::region::IsoRegion;

fn get_instance() -> InstanceUrl {
    InstanceUrl::new("https://vid.puffyan.us".parse().unwrap(), "/api/v1")
}

pub mod fetch_and_parse {
    use super::*;

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

    #[tokio::test]
    async fn playlist_fetch_and_parse() {
        let res = REQWEST_INVIDIOUS
            .get_playlist_info(
                &get_instance(),
                PlaylistInfoParams {
                    playlist_id: "PLm3v_EFg1EILrg_6fHX6kt2iEoIbFu6n4",
                    page: 1,
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn channel_fetch_and_parse() {
        let res = REQWEST_INVIDIOUS
            .get_channel_info(
                &get_instance(),
                ChannelInfoParams {
                    channel_id: "UCC3uNCR99Xw7uy2Gs4BwTrw",
                },
            )
            .await;
        println!("{res:?}");
        assert!(res.is_ok());
    }
}
