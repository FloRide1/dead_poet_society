use rocket::http::Status;
use rocket::{get, post, delete, patch};
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;

use crate::models::circle::{CircleModel, NewCircle};
use crate::Db;


#[get("/")]
pub async fn list_circles(db: Db) -> Result<Json<Vec<CircleModel>>, Status> {
    let res = CircleModel::list_circles(&db).await;

    match res {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/<id>")]
pub async fn get_circle(db: Db, id: i32) -> Option<Json<CircleModel>> {
    CircleModel::get_circle(&db, id).await.map(Json)
    // TODO: Add list of circles
}

#[post("/", format = "application/json", data = "<new_circle>")]
pub async fn new_circle(db: Db, new_circle: Json<NewCircle>) -> Result<Created<Json<CircleModel>>, Status> {
    let res: Result<CircleModel, diesel::result::Error> = CircleModel::new_circle(&db, new_circle.into_inner()).await;

    match res {
        Ok(res) => Ok(Created::new("/").body(Json(res))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[patch("/<id>", format = "application/json", data = "<new_circle>")]
pub async fn edit_circle(db: Db, id: i32, new_circle: Json<NewCircle>) -> Result<NoContent, Status> {
    let res: Result<usize, diesel::result::Error> = CircleModel::edit_circle(&db, id, new_circle.into_inner()).await;

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => Ok(NoContent),
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<id>")]
pub async fn delete_circle(db: Db, id: i32) -> Result<NoContent, Status>
{
    let res: Result<usize, diesel::result::Error> = CircleModel::delete_circle(&db, id).await;

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => Ok(NoContent),
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
