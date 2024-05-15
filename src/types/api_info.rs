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
#[serde(rename = "camelCase")]
pub struct SoftwareVersion {
    pub name: String,
    pub version: String,
    pub branch: String,
    pub metadata: Metadata,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Metadata {
    pub updated_at: i64,
    pub last_channel_refreshed_at: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct UsageStats {
    pub users: UserUsageStats,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct UserUsageStats {
    pub total: i32,
    pub active_halfyear: i32,
    pub active_month: i32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct InvidiousStats {
    pub version: String,
    pub software: SoftwareVersion,
    pub open_registrations: bool,
    pub usage: UsageStats,
    pub metadata: Metadata,
}
