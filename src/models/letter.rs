use std::fmt;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::Db;
use crate::api::requests::letter_request::LetterRequest;
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = letter)]
pub struct NewLetter {
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

impl fmt::Display for LetterModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Letter {name}>", name = self.subject)
    }
}

impl LetterModel {
    pub async fn get_letter(db: &Db, id: i32) -> Option<Self> {
        db.run(move |conn| { 
            letter::table
                .filter(letter::id.eq(id))
                .first::<Self>(conn)
        }).await.ok()
    }

    pub async fn new_letter(db: &Db, new_letter: NewLetter) -> Result<Self, diesel::result::Error>
    {
        db.run(move |conn| {
            diesel::insert_into(letter::table)
                .values(new_letter)
                .get_result(conn)
        }).await
    }

    pub async fn edit_letter(db: &Db, id: i32, new_letter: NewLetter) -> Result<usize, diesel::result::Error> {
        db.run(move |conn| {
            diesel::update(letter::table)
                .filter(letter::id.eq(id))
                .set(new_letter)
                .execute(conn)
        }).await
    }

    pub async fn delete_letter(db: &Db, id: i32) -> Result<usize, diesel::result::Error>
    {
        db.run(move |conn| {
            diesel::delete(letter::table)
                .filter(letter::id.eq(id))
                .execute(conn)
        }).await
    }
}

impl NewLetter {
    pub fn new(circle_id: i32, writer_id: i32, new_letter: LetterRequest) -> Self {
        Self {
            subject: new_letter.subject,
            content: new_letter.content,
            writer_id,
            circle_id,
            reply_id: None
        }
    }
}
