use crate::types::common;
use serde::{Deserialize, Serialize};

/*
{
    type: "playlist",
    title: String,
    playlistId: String,
    playlistThumbnail: String,
    author: String,
    authorId: String,
    authorUrl: String,
    authorVerified: Boolean,

    videoCount: Int32,
    videos: [
      {
        title: String,
        videoId: String,
        lengthSeconds: Int32,
        videoThumbnails: [
          {
            quality: String,
            url: String,
            width: Int32,
            height: Int32
          }
        ]
      }
    ]
  }
*/
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchPlaylistUnit {
    #[serde(rename = "type")]
    pub o_type: String,
    pub title: String,
    pub playlist_id: String,
    pub playlist_thumbnail: String,
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub author_verified: bool,

    pub video_count: i32,
    pub videos: Vec<common::PlaylistVideoObject>,
}
