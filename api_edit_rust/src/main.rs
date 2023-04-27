extern crate diesel;
extern crate dotenvy;
extern crate diesel_migrations;

pub mod models;
pub mod schema;
pub mod api;

use rocket::routes;
use rocket_sync_db_pools::database;

use dotenvy::dotenv;

use crate::api::writer as writer;
use crate::api::circle as circle;
use crate::api::letter as letter;

#[database("diesel")]
pub struct Db(diesel::PgConnection);

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Hello, world!");

    dotenv().ok();

    let _rocket = rocket::build()
        .attach(Db::fairing())
        .attach(rocket::fairing::AdHoc::on_ignite("Run Migrations", run_migrations))
        .mount("/writer", routes![writer::list_writers, writer::get_writer, writer::new_writer, writer::edit_writer, writer::delete_writer, writer::join_circle, writer::quit_circle])
        .mount("/circle", routes![circle::list_circles, circle::get_circle, circle::new_circle, circle::edit_circle, circle::delete_circle])
        .mount("/letter", routes![letter::list_letters, letter::get_letter, letter::post_letters, letter::delete_letter])
        .launch()
        .await?;

    Ok(())
}

async fn run_migrations(rocket: rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    Db::get_one(&rocket).await
        .expect("database connection")
        .run(|conn| { conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations"); })
        .await;
    rocket
}