use rocket::http::Status;
use rocket::{get, post, delete, patch};
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;

use crate::models::writer::{WriterModel, NewWriter};
use crate::models::writer_circle::WriterCircleModel;
use crate::Db;

#[get("/")]
pub async fn list_writers(db: Db) -> Result<Json<Vec<WriterModel>>, Status> {
    let res = WriterModel::list_writers(db).await;

    match res {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/<id>")]
pub async fn get_writer(db: Db, id: i32) -> Option<Json<WriterModel>> {
    WriterModel::get_writer(db, id).await.map(Json)

    // TODO: Add list of circles
}

#[post("/", format = "application/json", data = "<new_writer>")]
pub async fn new_writer(db: Db, new_writer: Json<NewWriter>) -> Result<Created<Json<WriterModel>>, Status> {
    let res: Result<WriterModel, diesel::result::Error> = WriterModel::new_writer(db, new_writer.into_inner()).await;

    match res {
        Ok(res) => Ok(Created::new("/").body(Json(res))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[patch("/<id>", format = "application/json", data = "<new_writer>")]
pub async fn edit_writer(db: Db, id: i32, new_writer: Json<NewWriter>) -> Result<NoContent, Status> {
    let res: Result<usize, diesel::result::Error> = WriterModel::edit_writer(db, id, new_writer.into_inner()).await;

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
    let res: Result<usize, diesel::result::Error> = WriterModel::delete_writer(db, id).await;

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => Ok(NoContent),
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/<writer_id>/circle/<circle_id>")]
pub async fn join_circle(db: Db, writer_id: i32, circle_id: i32) -> Status {
    let res = WriterCircleModel::new(db, writer_id, circle_id).await;

    match res {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError
    }
}

#[delete("/<writer_id>/circle/<circle_id>")]
pub async fn quit_circle(db: Db, writer_id: i32, circle_id: i32) -> Status {
    let res = WriterCircleModel::delete(db, writer_id, circle_id).await;

    match res {
        Ok(affected) if affected == 1 => Status::NoContent,
        Ok(_) => Status::NotFound,
        Err(_) => Status::InternalServerError
    }
}
