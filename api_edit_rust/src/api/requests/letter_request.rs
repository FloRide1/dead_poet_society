use serde::{self, Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LetterRequest {
    #[serde(rename = "subject")]
    pub subject: String,

    #[serde(rename = "content")]
    pub content: String,
}
