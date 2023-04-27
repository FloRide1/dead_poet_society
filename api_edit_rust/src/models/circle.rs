use std::fmt;

use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::{schema::circle, Db};

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

#[derive(Debug, PartialEq, Serialize, Deserialize, Insertable, AsChangeset)]
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

impl CircleModel {
    pub async fn list_circles(db: &Db) -> Result<Vec<CircleModel>, diesel::result::Error>{
        db.run(move |conn| {
            circle::table.load::<CircleModel>(conn)
        }).await

    }

    pub async fn get_circle(db: &Db, id: i32) -> Option<Self> {
        db.run(move |conn| { 
            circle::table
                .filter(circle::id.eq(id))
                .first::<Self>(conn)
        }).await.ok()
    }

    pub async fn new_circle(db: &Db, new_circle: NewCircle) -> Result<CircleModel, diesel::result::Error>
    {
        db.run(move |conn| {
            diesel::insert_into(circle::table)
                .values(new_circle)
                .get_result(conn)
        }).await
    }

    pub async fn edit_circle(db: &Db, id: i32, new_circle: NewCircle) -> Result<usize, diesel::result::Error> {
        db.run(move |conn| {
            diesel::update(circle::table)
                .filter(circle::id.eq(id))
                .set(new_circle)
                .execute(conn)
        }).await
    }

    pub async fn delete_circle(db: &Db, id: i32) -> Result<usize, diesel::result::Error>
    {
        db.run(move |conn| {
            diesel::delete(circle::table)
                .filter(circle::id.eq(id))
                .execute(conn)
        }).await
    }
}
