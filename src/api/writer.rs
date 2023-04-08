use diesel::{QueryDsl, RunQueryDsl};
use rocket::get;
use rocket::serde::json::Json;

use crate::models::writer::WriterModel;
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
