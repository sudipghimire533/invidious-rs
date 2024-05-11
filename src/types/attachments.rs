use super::common;
use serde::{Deserialize, Serialize};

/*
* ImageAttachment
{
    "type": "image",
    "imageThumbnails": ImageObject[]
}
*/
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageAttachment {
    #[serde(rename = "image")]
    o_type: String,
    image_thumbnails: Vec<common::ImageObject>,
}
/*
* MultiImageAttachment
*{
    "type": "multiImage",
    "images": ImageObject[][]
}
*/
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiImageAttachment {
    #[serde(rename = "type")]
    o_type: String,
    images: Vec<Vec<common::ImageObject>>,
}

/*
* PoolAttachment
*{
    "type": "poll",
    "totalVotes": Number,
    "choices": {
        "text": String,
        "image?": ImageObject[]
    }[]

}
*/
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PollAttachment {
    #[serde(rename = "type")]
    o_type: String,
    total_votes: i64,
    choices: Vec<PoolChoice>,
}

/*
PoolChoice
    {
        "text": String,
        "image?": ImageObject[]
    }
*/
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolChoice {
    text: String,
    image: Option<Vec<common::ImageObject>>,
}

/*
* Unknown
* {
*   type: "unknown",
*   "error": String
* }
*/
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnknownAttachment {
    #[serde(rename = "type")]
    o_type: String,
    error: String,
}

pub type VideoAttachment = common::VideoObject;
pub type PlaylistAttachment = common::PlaylistObject;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Attachment {
    VideoAttachment(VideoAttachment),
    ImageAttachment(ImageAttachment),
    MultiImageAttachment(MultiImageAttachment),
    PollAttachment(PollAttachment),
    PlaylistAttachment(PlaylistAttachment),
    UnknownAttachment(UnknownAttachment),
}
