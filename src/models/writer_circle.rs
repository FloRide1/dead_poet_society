use std::fmt;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::{schema::writercircle, Db};

use crate::models::circle::CircleModel;
use crate::models::writer::WriterModel;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Queryable, Identifiable, Insertable, Associations,
)]
#[diesel(belongs_to(WriterModel, foreign_key = writer_id))]
#[diesel(belongs_to(CircleModel, foreign_key = circle_id))]
#[diesel(primary_key(writer_id, circle_id))]
#[diesel(table_name = writercircle)]
pub struct WriterCircleModel {
    #[diesel(column_name = writer_id)]
    pub writer_id: i32,

    #[diesel(column_name = circle_id)]
    pub circle_id: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = writercircle)]
pub struct NewWriterCircle {
    #[diesel(column_name = circle_id)]
    pub circle_id: i32,

    #[diesel(column_name = writer_id)]
    pub writer_id: i32,
}

impl fmt::Display for WriterCircleModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<WriterCircle W:{writer} C:{circle}>",
            writer = self.writer_id,
            circle = self.circle_id
        )
    }
}

impl WriterCircleModel {
    pub async fn new(db: &Db, writer_id: i32, circle_id: i32) -> Result<usize, diesel::result::Error> {
        db.run(move |conn| {
            diesel::insert_into(writercircle::table)
                .values(Self { writer_id, circle_id })
                .execute(conn)
        }).await
    }

    pub async fn get_writer_circles(db: &Db, writer_id: i32) -> Result<Vec<i32>, diesel::result::Error> {
        db.run(move |conn| {
            writercircle::table
                .filter(writercircle::writer_id.eq(writer_id))
                .select(writercircle::circle_id)
                .load::<i32>(conn)
        }).await
    }

    pub async fn get_circle_writers(db: &Db, circle_id: i32) -> Result<Vec<i32>, diesel::result::Error> {
        db.run(move |conn| {
            writercircle::table
                .filter(writercircle::circle_id.eq(circle_id))
                .select(writercircle::writer_id)
                .load::<i32>(conn)
        }).await
    }

    pub async fn delete(db: &Db, writer_id: i32, circle_id: i32) -> Result<usize, diesel::result::Error> {
        db.run(move |conn|  {
            diesel::delete(writercircle::table)
                .filter(writercircle::circle_id.eq(circle_id))
                .filter(writercircle::writer_id.eq(writer_id))
                .execute(conn)
        }).await
    }
}
