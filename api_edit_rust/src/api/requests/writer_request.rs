use serde::{self, Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct WriterRequest {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "pseudo")]
    pub pseudo: String,
}
