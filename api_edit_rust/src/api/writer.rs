use rocket::http::Status;
use rocket::{post, delete, patch};
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;

use crate::api::requests::writer_request::WriterRequest;
use crate::api::user::AuthUser;
use crate::models::writer::{WriterModel, NewWriter};
use crate::models::writer_circle::{WriterCircleModel, NewWriterCircle};
use crate::Db;

#[post("/", format = "application/json", data = "<new_writer>")]
pub async fn new_writer(db: Db, user: AuthUser, new_writer: Json<WriterRequest>) -> Result<Created<Json<WriterModel>>, Status> {
    let new_writer = new_writer.into_inner();

    crate::mqtt::mqtt_core::mqtt_publish("new_writer", &new_writer);
    crate::mqtt::mqtt_core::mqtt_publish_json("new_writer_json", &new_writer);

    if WriterModel::get_writer_by_name(&db, user.user.to_string()).await.is_some() {
        return Err(Status::Conflict);
    }
    
    let res: Result<WriterModel, diesel::result::Error> = WriterModel::new_writer(&db, NewWriter { title: new_writer.title, name: user.user, pseudo: new_writer.pseudo}).await;

    match res {
        Ok(res) => {
            crate::mqtt::mqtt_core::mqtt_publish("new_writer_confirmed", &res);
            crate::mqtt::mqtt_core::mqtt_publish_json("new_writer_confirmed_json", &res);

            Ok(Created::new("/").body(Json(res)))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[patch("/", format = "application/json", data = "<edit_writer>")]
pub async fn edit_writer(db: Db, user: AuthUser,  edit_writer: Json<WriterRequest>) -> Result<NoContent, Status> {
    let edit_writer = edit_writer.into_inner();

    crate::mqtt::mqtt_core::mqtt_publish("edit_writer", &edit_writer);
    crate::mqtt::mqtt_core::mqtt_publish_json("edit_writer_json", &edit_writer);

    let edit_writer_clone = edit_writer.clone();
    let writer = WriterModel::get_or_create_user(&db, user.user).await;
    if writer.is_err() {
        return Err(Status::InternalServerError);
    }
    let writer = writer.unwrap();

    let res: Result<usize, diesel::result::Error> = WriterModel::edit_writer(&db, writer.id, NewWriter { title: edit_writer.title, name: writer.name, pseudo: edit_writer.pseudo }).await;

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

#[post("/circle/<circle_id>")]
pub async fn join_circle(db: Db, user: AuthUser, circle_id: i32) -> Status {
    let writer = WriterModel::get_or_create_user(&db, user.user.to_string()).await.unwrap();

    let new_writer_circle = NewWriterCircle::new(writer.id, circle_id);

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

#[delete("/circle/<circle_id>")]
pub async fn quit_circle(db: Db, user: AuthUser, circle_id: i32) -> Status {
    let writer = WriterModel::get_or_create_user(&db, user.user.to_string()).await.unwrap();

    let writer_circle = WriterCircleModel::new(writer.id, circle_id);

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
