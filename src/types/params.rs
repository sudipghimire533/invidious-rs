use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QueryResultDuration {
    Short,
    Long,
    Medium,
}

impl QueryResultDuration {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Short => "short",
            Self::Long => "long",
            Self::Medium => "medium",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QueryResultFeatures {
    Hd,
    Subtitles,
    CeativeCommons,
    ThreeD,
    Live,
    Purchased,
    FourK,
    Three60,
    Location,
    Hdr,
    Vr180,
}

impl QueryResultFeatures {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hd => "hd",
            Self::Subtitles => "subtitles",
            Self::CeativeCommons => "creativeCommons",
            Self::ThreeD => "3d",
            Self::Live => "live",
            Self::Purchased => "purchased",
            Self::FourK => "4k",
            Self::Three60 => "360",
            Self::Location => "location",
            Self::Hdr => "hdr",
            Self::Vr180 => "vr180",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchResultType {
    Video,
    Playlist,
    Channel,
    Movie,
    Show,
    All,
}

impl SearchResultType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Video => "video",
            Self::Playlist => "playlist",
            Self::Channel => "channel",
            Self::Movie => "movie",
            Self::Show => "show",
            Self::All => "all",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QueryResultDate {
    Hour,
    Today,
    Week,
    Month,
    Year,
}

impl QueryResultDate {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hour => "hour",
            Self::Today => "today",
            Self::Week => "week",
            Self::Month => "month",
            Self::Year => "year",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlaylistSortingOption {
    Oldest,
    Newest,
    Last,
}

impl PlaylistSortingOption {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Oldest => "oldest",
            Self::Newest => "newest",
            Self::Last => "last",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortingOption {
    Relavence,
    Rating,
    UploadDate,
    ViewCount,
}

impl SortingOption {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Relavence => "relevance",
            Self::Rating => "rating",
            Self::UploadDate => "uploadDate",
            Self::ViewCount => "viewCount",
        }
    }
}

/// Comment source
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommentSource {
    Youtube,
    Reddit,
}

impl CommentSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Reddit => "reddit",
            Self::Youtube => "youtube",
        }
    }
}

/// Sort comments
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommentSorting {
    Top,
    New,
}

impl CommentSorting {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::New => "new",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContentCategory {
    Music,
    Gaming,
    News,
    Movies,
}

impl ContentCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Music => "music",
            Self::Gaming => "gaming",
            Self::News => "news",
            Self::Movies => "movies",
        }
    }
}
