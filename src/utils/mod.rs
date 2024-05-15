use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UntaggeBinary<First, Second> {
    Primary(First),
    Secondary(Second),
}
