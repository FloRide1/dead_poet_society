use std::fmt;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::schema::circle;

#[derive(
    Debug, PartialEq, Serialize, Deserialize, Queryable, Identifiable, Insertable, AsChangeset,
)]
#[diesel(table_name = circle)]
pub struct CircleModel {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = name)]
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = circle)]
pub struct NewCircle {
    #[diesel(column_name = name)]
    pub name: String,
}

impl fmt::Display for CircleModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Circle {name}>", name = self.name)
    }
}

impl CircleModel {}
