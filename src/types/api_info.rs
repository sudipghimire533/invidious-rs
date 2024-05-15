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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_deserilization() {
        let version_str = r#"{"version":"2.0","software":{"name":"invidious","version":"2024.04.27-eda7444","branch":"master"},"openRegistrations":true,"usage":{"users":{"total":22250,"activeHalfyear":4407,"activeMonth":647}},"metadata":{"updatedAt":1715795523,"lastChannelRefreshedAt":1715794967},"playback":{"totalRequests":50,"successfulRequests":0,"ratio":0.0}}"#;
        let res = serde_json::from_str::<super::InvidiousStats>(version_str);
        let expected = InvidiousStats {
            version: "2.0".to_string(),
            software: SoftwareVersion {
                name: "invidious".to_string(),
                version: "2024.04.27-eda7444".to_string(),
                branch: "master".to_string(),
            },
            open_registrations: true,
            usage: UsageStats {
                users: UserUsageStats {
                    total: 22250,
                    active_halfyear: 4407,
                    active_month: 647,
                },
            },
            metadata: Metadata {
                updated_at: 1715795523,
                last_channel_refreshed_at: 1715794967,
            },
            playback: Playback {
                total_requests: Some(50),
                successful_requests: Some(0),
                ratio: Some(0.0.into()),
            },
        };
        match res {
            Ok(e) if e != expected => {
                panic!("Expected: {expected:?}. Got: {e:?}")
            }
            Err(e) => {
                panic!("Error: {e}")
            }
            Ok(_) => {}
        }
    }
}
