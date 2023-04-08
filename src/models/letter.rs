use std::fmt;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::models::circle::CircleModel;
use crate::models::writer::WriterModel;
use crate::schema::letter;

#[derive(
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    Queryable,
    Identifiable,
    Insertable,
    AsChangeset,
    Associations,
)]
#[diesel(belongs_to(WriterModel, foreign_key = writer_id))]
#[diesel(belongs_to(CircleModel, foreign_key = circle_id))]
#[diesel(belongs_to(LetterModel, foreign_key = reply_id))]
#[diesel(table_name = letter)]
pub struct LetterModel {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = subject)]
    pub subject: String,

    #[diesel(column_name = content)]
    pub content: String,

    #[diesel(column_name = circle_id)]
    pub circle_id: i32,

    #[diesel(column_name = writer_id)]
    pub writer_id: i32,

    #[diesel(column_name = reply_id)]
    pub reply_id: Option<i32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = letter)]
pub struct NewLetter {
    #[diesel(column_name = subject)]
    pub subject: String,

    #[diesel(column_name = content)]
    pub content: String,
}

impl fmt::Display for LetterModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Letter {name}>", name = self.subject)
    }
}
