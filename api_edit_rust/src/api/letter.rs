use rocket::http::Status;
use rocket::{post, delete};
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;

use crate::Db;
use crate::models::letter::{LetterModel, NewLetter};
use crate::models::writer::WriterModel;

use super::requests::letter_request::LetterRequest;
use super::user::AuthUser;

#[post("/circle/<circle_id>", format = "application/json", data = "<new_letter>")]
pub async fn post_letters(db: Db, circle_id: i32, auth: AuthUser, new_letter: Json<LetterRequest>) -> Result<Created<Json<LetterModel>>, Status> {
    let writer = WriterModel::get_or_create_user(&db, auth.user).await.unwrap();

    let new_letter = NewLetter::new(circle_id, writer.id, new_letter.into_inner());

    crate::mqtt::mqtt_core::mqtt_publish("new_letter", &new_letter);
    crate::mqtt::mqtt_core::mqtt_publish_json("new_letter_json", &new_letter);

    let res: Result<LetterModel, diesel::result::Error> = LetterModel::new_letter(&db, new_letter).await;

    match res {
        Ok(res) => {
            crate::mqtt::mqtt_core::mqtt_publish("new_letter_confirmed", &res);
            crate::mqtt::mqtt_core::mqtt_publish_json("new_letter_confirmed_json", &res);

            Ok(Created::new("/").body(Json(res)))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/<id>")]
pub async fn delete_letter(db: Db, auth: AuthUser, id: i32) -> Result<NoContent, Status>
{
    let writer = WriterModel::get_writer_by_name(&db, auth.user).await;
    if writer.is_none() {
        return Err(Status::Unauthorized);
    }

    let writer = writer.unwrap();
    match LetterModel::get_letter(&db, id).await {
        Some(letter) => { 
            if letter.writer_id != writer.id 
            {
                return Err(Status::Unauthorized);
            }
        },
        None => return Err(Status::NotFound),
    }

    crate::mqtt::mqtt_core::mqtt_publish("delete_letter", id);
    crate::mqtt::mqtt_core::mqtt_publish_json("delete_letter_json", id);

    let res: Result<usize, diesel::result::Error> = LetterModel::delete_letter(&db, id).await;

    // TODO: Add Unauthorised ?
    match res {
        Ok(affected) if affected == 1 => {
            crate::mqtt::mqtt_core::mqtt_publish("delete_letter_confirmed", id);
            crate::mqtt::mqtt_core::mqtt_publish_json("delete_letter_confirmed_json", id);

            Ok(NoContent)
        },
        Ok(_) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
