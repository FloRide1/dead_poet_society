use diesel::{QueryDsl, RunQueryDsl};
use rocket::http::Status;
use rocket::{get, post, delete, patch};
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;

use crate::models::circle::{CircleModel, NewCircle};
use crate::schema::circle;
use crate::Db;

use crate::diesel::prelude::*;

#[get("/")]
pub async fn list_circles(db: Db) -> Result<Json<Vec<CircleModel>>, Status> {
    let res = db.run(move |conn| {
        circle::table.load::<CircleModel>(conn)
    }).await;

    match res {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/<id>")]
pub async fn get_circle(db: Db, id: i32) -> Option<Json<CircleModel>> {
    db.run(move |conn| { 
        circle::table
            .filter(circle::id.eq(id))
            .first(conn)
    }).await.map(Json).ok()

    // TODO: Add list of circles
}

#[post("/", format = "application/json", data = "<new_circle>")]
pub async fn new_circle(db: Db, new_circle: Json<NewCircle>) -> Result<Created<Json<CircleModel>>, Status> {
    let res: Result<CircleModel, diesel::result::Error> = db.run(move |conn| {
        diesel::insert_into(circle::table)
            .values(&*new_circle)
            .get_result(conn)
    }).await;

    match res {
        Ok(res) => Ok(Created::new("/").body(Json(res))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[patch("/<id>", format = "application/json", data = "<new_circle>")]
pub async fn edit_circle(db: Db, id: i32, new_circle: Json<NewCircle>) -> Result<NoContent, Status> {
    let res: Result<usize, diesel::result::Error> = db.run(move |conn| {
        diesel::update(circle::table)
            .filter(circle::id.eq(id))
            .set(&*new_circle)
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
pub async fn delete_circle(db: Db, id: i32) -> Result<NoContent, Status>
{
    let res: Result<usize, diesel::result::Error>  = db.run(move |conn| {
        diesel::delete(circle::table)
            .filter(circle::id.eq(id))
            .execute(conn)
    }).await;

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => Ok(NoContent),
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
