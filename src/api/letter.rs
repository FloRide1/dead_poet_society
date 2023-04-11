use rocket::http::Status;
use rocket::{get, post, delete};
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;

use crate::Db;
use crate::models::letter::{LetterModel, NewLetter};

use super::requests::letter_request::LetterRequest;

#[get("/<id>")]
pub async fn get_letter(db: Db, id: i32) -> Option<Json<LetterModel>> {
    LetterModel::get_letter(&db, id).await.map(Json)
}

#[post("/circle/<circle_id>/writer/<writer_id>", format = "application/json", data = "<new_letter>")]
pub async fn post_letters(db: Db, circle_id: i32, writer_id: i32, new_letter: Json<LetterRequest>) -> Result<Created<Json<LetterModel>>, Status> {

    let res: Result<LetterModel, diesel::result::Error> = LetterModel::new_letter(&db, NewLetter::new(circle_id, writer_id, new_letter.into_inner())).await;

    match res {
        Ok(res) => Ok(Created::new("/").body(Json(res))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<id>")]
pub async fn delete_letter(db: Db, id: i32) -> Result<NoContent, Status>
{
    let res: Result<usize, diesel::result::Error> = LetterModel::delete_letter(&db, id).await;

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => Ok(NoContent),
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
