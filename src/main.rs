extern crate diesel;
extern crate dotenvy;

pub mod models;
pub mod schema;
pub mod api;

use rocket::routes;
use rocket_sync_db_pools::database;

use dotenvy::dotenv;

#[database("diesel")]
pub struct Db(diesel::PgConnection);

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Hello, world!");

    dotenv().ok();

    let _rocket = rocket::build()
        .attach(Db::fairing())
        .mount("/writer", routes![crate::api::writer::list_writers, crate::api::writer::get_writer, crate::api::writer::new_writer])
        .launch()
        .await?;

    Ok(())
}
