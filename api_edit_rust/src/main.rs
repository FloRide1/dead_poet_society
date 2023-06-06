extern crate diesel;
extern crate dotenvy;
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;

pub mod models;
pub mod schema;
pub mod api;
pub mod mqtt;

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

    let host = std::env::var("MQTT_HOST").expect("MQTT_HOST is not set");
    let port = std::env::var("MQTT_PORT").expect("MQTT_PORT is not set")
                    .parse::<u16>().expect("MQTT_PORT is not a number");

    mqtt::mqtt_core::mqtt_login(&host, port);

    let _rocket = rocket::build()
        .attach(Db::fairing())
        .attach(rocket::fairing::AdHoc::on_ignite("Run Migrations", run_migrations))
        .mount("/writer", routes![writer::new_writer, writer::edit_writer, writer::join_circle, writer::quit_circle])
        .mount("/circle", routes![circle::new_circle, circle::edit_circle, circle::delete_circle])
        .mount("/letter", routes![letter::post_letters, letter::delete_letter])
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
