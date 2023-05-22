use rocket::http::Status;
use rocket::{get, post, delete, patch};
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;

use crate::models::writer::{WriterModel, NewWriter};
use crate::models::writer_circle::{WriterCircleModel, NewWriterCircle};
use crate::Db;

use super::responses::writer_response::WriterResponse;

#[get("/")]
pub async fn list_writers(db: Db) -> Result<Json<Vec<WriterModel>>, Status> {
    let res = WriterModel::list_writers(&db).await;

    match res {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/<id>")]
pub async fn get_writer(db: Db, id: i32) -> Option<Json<WriterResponse>> {
    let model = WriterModel::get_writer(&db, id).await;
    model.as_ref()?;

    let circles = WriterCircleModel::get_writer_circles(&db, id).await;
    if circles.is_err() {
        return Option::None;
    }

    Some(Json(WriterResponse::new(model.unwrap(), circles.unwrap_or(vec![]))))
}

#[post("/", format = "application/json", data = "<new_writer>")]
pub async fn new_writer(db: Db, new_writer: Json<NewWriter>) -> Result<Created<Json<WriterModel>>, Status> {
    let new_writer: NewWriter = new_writer.into_inner();

    crate::mqtt::mqtt_core::mqtt_publish("new_writer", &new_writer);
    crate::mqtt::mqtt_core::mqtt_publish_json("new_writer_json", &new_writer);

    let res: Result<WriterModel, diesel::result::Error> = WriterModel::new_writer(&db, new_writer).await;

    match res {
        Ok(res) => {
            crate::mqtt::mqtt_core::mqtt_publish("new_writer_confirmed", &res);
            crate::mqtt::mqtt_core::mqtt_publish_json("new_writer_confirmed_json", &res);

            Ok(Created::new("/").body(Json(res)))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[patch("/<id>", format = "application/json", data = "<edit_writer>")]
pub async fn edit_writer(db: Db, id: i32, edit_writer: Json<NewWriter>) -> Result<NoContent, Status> {
    let edit_writer = edit_writer.into_inner();

    crate::mqtt::mqtt_core::mqtt_publish("edit_writer", &edit_writer);
    crate::mqtt::mqtt_core::mqtt_publish_json("edit_writer_json", &edit_writer);

    let edit_writer_clone = edit_writer.clone();
    let res: Result<usize, diesel::result::Error> = WriterModel::edit_writer(&db, id, edit_writer).await;

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => {
            crate::mqtt::mqtt_core::mqtt_publish("edit_writer_confirmed", &edit_writer_clone);
            crate::mqtt::mqtt_core::mqtt_publish_json("edit_writer_confirmed_json", &edit_writer_clone);

            Ok(NoContent)
        },
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<id>")]
pub async fn delete_writer(db: Db, id: i32) -> Result<NoContent, Status>
{
    let res: Result<usize, diesel::result::Error> = WriterModel::delete_writer(&db, id).await;

    crate::mqtt::mqtt_core::mqtt_publish("delete_writer", id);
    crate::mqtt::mqtt_core::mqtt_publish_json("delete_writer_json", id);

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => { 
            crate::mqtt::mqtt_core::mqtt_publish("delete_writer_confirmed", id);
            crate::mqtt::mqtt_core::mqtt_publish_json("delete_writer_confirmed_json", id);

            Ok(NoContent)
        },
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/<writer_id>/circle/<circle_id>")]
pub async fn join_circle(db: Db, writer_id: i32, circle_id: i32) -> Status {
    let new_writer_circle = NewWriterCircle::new(writer_id, circle_id);

    crate::mqtt::mqtt_core::mqtt_publish("join_circle", &new_writer_circle);
    crate::mqtt::mqtt_core::mqtt_publish_json("join_circle_json", &new_writer_circle);

    let res = WriterCircleModel::create(&db, &new_writer_circle).await;


    match res {
        Ok(_) => { 
            crate::mqtt::mqtt_core::mqtt_publish("join_circle_confirmed", &new_writer_circle);
            crate::mqtt::mqtt_core::mqtt_publish_json("join_circle_confirmed_json", &new_writer_circle);

            Status::Created
        },
        Err(_) => Status::Conflict
    }
}

#[delete("/<writer_id>/circle/<circle_id>")]
pub async fn quit_circle(db: Db, writer_id: i32, circle_id: i32) -> Status {
    let writer_circle = WriterCircleModel::new(writer_id, circle_id);

    crate::mqtt::mqtt_core::mqtt_publish("quit_circle", &writer_circle);
    crate::mqtt::mqtt_core::mqtt_publish_json("quit_circle_json", &writer_circle);

    let res = writer_circle.delete(&db).await;

    match res {
        Ok(affected) if affected == 1 => {
            crate::mqtt::mqtt_core::mqtt_publish("quit_circle_confirmed", &writer_circle);
            crate::mqtt::mqtt_core::mqtt_publish_json("quit_circle_confirmed_json", &writer_circle);


            Status::NoContent
        },
        Ok(_) => Status::NotFound,
        Err(_) => Status::InternalServerError
    }
}
