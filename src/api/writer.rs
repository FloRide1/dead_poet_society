use diesel::{QueryDsl, RunQueryDsl};
use rocket::http::Status;
use rocket::{get, post, delete, patch};
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;

use crate::models::writer::{WriterModel, NewWriter};
use crate::schema::writer;
use crate::Db;

use crate::diesel::prelude::*;

#[get("/")]
pub async fn list_writers(db: Db) -> Result<Json<Vec<WriterModel>>, Status> {
    let res = db.run(move |conn| {
        writer::table.load::<WriterModel>(conn)
    }).await;

    match res {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/<id>")]
pub async fn get_writer(db: Db, id: i32) -> Option<Json<WriterModel>> {
    db.run(move |conn| { 
        writer::table
            .filter(writer::id.eq(id))
            .first(conn)
    }).await.map(Json).ok()

    // TODO: Add list of circles
}

#[post("/", format = "application/json", data = "<new_writer>")]
pub async fn new_writer(db: Db, new_writer: Json<NewWriter>) -> Result<Created<Json<WriterModel>>, Status> {
    let res: Result<WriterModel, diesel::result::Error> = db.run(move |conn| {
        diesel::insert_into(writer::table)
            .values(&*new_writer)
            .get_result(conn)
    }).await;

    match res {
        Ok(res) => Ok(Created::new("/").body(Json(res))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[patch("/<id>", format = "application/json", data = "<new_writer>")]
pub async fn edit_writer(db: Db, id: i32, new_writer: Json<NewWriter>) -> Result<NoContent, Status> {
    let res: Result<usize, diesel::result::Error> = db.run(move |conn| {
        diesel::update(writer::table)
            .filter(writer::id.eq(id))
            .set(&*new_writer)
            .execute(conn)
    }).await;

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => Ok(NoContent),
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<id>")]
pub async fn delete_writer(db: Db, id: i32) -> Result<NoContent, Status>
{
    let res: Result<usize, diesel::result::Error>  = db.run(move |conn| {
        diesel::delete(writer::table)
            .filter(writer::id.eq(id))
            .execute(conn)
    }).await;

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => Ok(NoContent),
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
