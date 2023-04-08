use std::fmt;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::writercircle;

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

impl WriterCircleModel {}
