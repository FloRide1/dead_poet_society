use serde::{self, Deserialize, Serialize};

use crate::models::writer::WriterModel;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WriterResponse {
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "pseudo")]
    pub pseudo: String,

    #[serde(rename = "circles")]
    pub circles: Vec<i32>,
}

impl WriterResponse {
    pub fn new(model: WriterModel, circles: Vec<i32>) -> Self {
        Self {
            id: model.id,
            title: model.title,
            name: model.name,
            pseudo: model.pseudo,
            circles,
        }
    }
}
