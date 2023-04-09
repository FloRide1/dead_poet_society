use std::fmt;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::writer;

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

#[derive(Debug, PartialEq, Serialize, Deserialize, Insertable, AsChangeset)]
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

impl WriterModel {}
