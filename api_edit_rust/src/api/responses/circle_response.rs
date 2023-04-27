use serde::{self, Deserialize, Serialize};

use crate::models::circle::CircleModel;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CircleResponse {
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "writers")]
    pub writers: Vec<i32>,
}

impl CircleResponse {
    pub fn new(model: CircleModel, writers: Vec<i32>) -> Self {
        Self {
            id: model.id,
            name: model.name,
            writers,
        }
    }
}
