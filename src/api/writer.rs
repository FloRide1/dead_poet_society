use diesel::{QueryDsl, RunQueryDsl};
use rocket::{get, post, delete};
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;

use crate::models::writer::{WriterModel, NewWriter};
use crate::schema::writer;
use crate::Db;

use crate::diesel::prelude::*;


type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
pub async fn list_writers(db: Db) -> Result<Json<Vec<WriterModel>>> {
    Ok(Json(db.run(move |conn| {
        writer::table.load::<WriterModel>(conn)
    }).await?))
}

#[get("/<id>")]
pub async fn get_writer(db: Db, id: i32) -> Option<Json<WriterModel>> {
    db.run(move |conn| { 
        writer::table
            .filter(writer::id.eq(id))
            .first(conn)
    }).await.map(Json).ok()
}

#[post("/", format = "application/json", data = "<new_writer>")]
pub async fn new_writer(db: Db, new_writer: Json<NewWriter>) -> Result<Created<Json<WriterModel>>> {
    let res: WriterModel = db.run(move |conn| {
        diesel::insert_into(writer::table)
            .values(&*new_writer)
            .get_result(conn)
    }).await?;

    Ok(Created::new("/").body(Json(res)))
}

#[delete("/<id>")]
pub async fn delete_writer(db: Db, id: i32) -> Result<NoContent>
{
    db.run(move |conn| {
        diesel::delete(writer::table)
            .filter(writer::id.eq(id))
            .execute(conn)
    }).await?;

    Ok(NoContent)
}
