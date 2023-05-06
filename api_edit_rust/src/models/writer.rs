use std::fmt;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::{schema::writer, Db};

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset,
)]
#[diesel(table_name = writer)]
pub struct WriterModel {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = title)]
    pub title: String,

    #[diesel(column_name = name)]
    pub name: String,

    #[diesel(column_name = pseudo)]
    pub pseudo: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Insertable, AsChangeset, Clone)]
#[diesel(table_name = writer)]
pub struct NewWriter {
    #[diesel(column_name = title)]
    pub title: String,

    #[diesel(column_name = name)]
    pub name: String,

    #[diesel(column_name = pseudo)]
    pub pseudo: String,
}

impl fmt::Display for WriterModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Writer {name}>", name = self.name)
    }
}

impl WriterModel {
    pub async fn list_writers(db: &Db) -> Result<Vec<WriterModel>, diesel::result::Error>{
        db.run(move |conn| {
            writer::table.load::<WriterModel>(conn)
        }).await

    }


    pub async fn get_writer(db: &Db, id: i32) -> Option<Self> {
        db.run(move |conn| { 
            writer::table
                .filter(writer::id.eq(id))
                .first::<Self>(conn)
        }).await.ok()
    }

    pub async fn new_writer(db: &Db, new_writer: NewWriter) -> Result<WriterModel, diesel::result::Error>
    {
        db.run(move |conn| {
            diesel::insert_into(writer::table)
                .values(new_writer)
                .get_result(conn)
        }).await
    }

    pub async fn edit_writer(db: &Db, id: i32, new_writer: NewWriter) -> Result<usize, diesel::result::Error> {
        db.run(move |conn| {
            diesel::update(writer::table)
                .filter(writer::id.eq(id))
                .set(new_writer)
                .execute(conn)
        }).await
    }

    pub async fn delete_writer(db: &Db, id: i32) -> Result<usize, diesel::result::Error>
    {
        db.run(move |conn| {
            diesel::delete(writer::table)
                .filter(writer::id.eq(id))
                .execute(conn)
        }).await
    }
}
