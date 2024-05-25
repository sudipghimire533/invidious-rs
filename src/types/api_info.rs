use serde::{Deserialize, Serialize};

/*
InvidiousStats
{
  "version": String,
  "software": {
    "name": "invidious",
    "version": String,
    "branch": String
  },
  "openRegistrations": Bool,
  "usage": {
    "users": {
      "total": Int32,
      "activeHalfyear": Int32,
      "activeMonth": Int32
    }
  },
  "metadata": {
    "updatedAt": Int64,
    "lastChannelRefreshedAt": Int64
  }
}
*/

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareVersion {
    pub name: String,
    pub version: String,
    pub branch: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub updated_at: i64,
    pub last_channel_refreshed_at: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageStats {
    pub users: UserUsageStats,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUsageStats {
    pub total: i32,
    pub active_halfyear: i32,
    pub active_month: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playback {
    total_requests: Option<i32>,
    successful_requests: Option<i32>,
    ratio: Option<ordered_float::OrderedFloat<f32>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvidiousStats {
    pub version: String,
    pub software: SoftwareVersion,
    pub open_registrations: bool,
    pub usage: UsageStats,
    pub playback: Playback,
    pub metadata: Metadata,
}
