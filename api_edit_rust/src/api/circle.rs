use rocket::http::Status;
use rocket::{get, post, delete, patch};
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;

use crate::models::circle::{CircleModel, NewCircle};
use crate::Db;
use crate::models::writer_circle::WriterCircleModel;

use super::responses::circle_response::CircleResponse;


#[get("/")]
pub async fn list_circles(db: Db) -> Result<Json<Vec<CircleModel>>, Status> {
    let res = CircleModel::list_circles(&db).await;

    match res {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/<id>")]
pub async fn get_circle(db: Db, id: i32) -> Option<Json<CircleResponse>> {
    let model = CircleModel::get_circle(&db, id).await;
    model.as_ref()?;

    let writers = WriterCircleModel::get_circle_writers(&db, id).await;
    if writers.is_err() {
        return Option::None;
    }

    Some(Json(CircleResponse::new(model.unwrap(), writers.unwrap_or(vec![]))))
}

#[post("/", format = "application/json", data = "<new_circle>")]
pub async fn new_circle(db: Db, new_circle: Json<NewCircle>) -> Result<Created<Json<CircleModel>>, Status> {
    let new_circle = new_circle.into_inner();

    crate::mqtt::mqtt_core::mqtt_publish("new_circle", &new_circle).await;
    crate::mqtt::mqtt_core::mqtt_publish_json("new_circle_json", &new_circle).await;

    let res: Result<CircleModel, diesel::result::Error> = CircleModel::new_circle(&db, new_circle).await;

    match res {
        Ok(res) => {
            crate::mqtt::mqtt_core::mqtt_publish("new_circle_confirmed", &res).await;
            crate::mqtt::mqtt_core::mqtt_publish_json("new_circle_confirmed_json", &res).await;

            Ok(Created::new("/").body(Json(res)))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[patch("/<id>", format = "application/json", data = "<edit_circle>")]
pub async fn edit_circle(db: Db, id: i32, edit_circle: Json<NewCircle>) -> Result<NoContent, Status> {
    let edit_circle = edit_circle.into_inner();

    crate::mqtt::mqtt_core::mqtt_publish("edit_circle", &edit_circle).await;
    crate::mqtt::mqtt_core::mqtt_publish_json("edit_circle_json", &edit_circle).await;

    let edit_circle_clone = edit_circle.clone();
    let res: Result<usize, diesel::result::Error> = CircleModel::edit_circle(&db, id, edit_circle).await;

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => {
            crate::mqtt::mqtt_core::mqtt_publish("edit_circle_confirmed", &edit_circle_clone).await;
            crate::mqtt::mqtt_core::mqtt_publish_json("edit_circle_confirmed_json", &edit_circle_clone).await;

            Ok(NoContent)
        },
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<id>")]
pub async fn delete_circle(db: Db, id: i32) -> Result<NoContent, Status>
{
    crate::mqtt::mqtt_core::mqtt_publish("delete_circle", id).await;
    crate::mqtt::mqtt_core::mqtt_publish_json("delete_circle_json", id).await;

    let res: Result<usize, diesel::result::Error> = CircleModel::delete_circle(&db, id).await;

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => {
            crate::mqtt::mqtt_core::mqtt_publish("delete_circle_confirmed", id).await;
            crate::mqtt::mqtt_core::mqtt_publish_json("delete_circle_json", id).await;

            Ok(NoContent)
        },
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
