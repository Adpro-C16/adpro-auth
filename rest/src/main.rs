#[macro_use]
extern crate rocket;

use autometrics::prometheus_exporter;
use dotenvy::dotenv;
use rocket_cors::AllowedOrigins;
// use shuttle_rocket::ShuttleRocket;

use sqlx::postgres::PgPoolOptions;
use std::env;

pub mod controller;

pub mod model;

#[get("/")]
pub fn index() -> &'static str {
    "Heymart C14 - Auth Service"
}

#[get("/metrics")]
pub fn metrics() -> String {
    prometheus_exporter::encode_to_string().unwrap()
}

// #[shuttle_runtime::main]
// async fn main(#[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore) -> ShuttleRocket {
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    prometheus_exporter::init();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    // let url = secrets.get("DATABASE_URL").unwrap();

    // secrets.into_iter().for_each(|(key, value)| {
    // std::env::set_var(key, value);
    // });

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&url)
        .await
        .unwrap();

    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let rocket = rocket::build()
        .manage(pool)
        .attach(cors)
        .mount(
            "/auth",
            routes![controller::auth::login, controller::auth::register,],
        )
        .mount("/user", routes![controller::user::get_user,])
        .mount("/", routes![index, metrics]);

    rocket.launch().await?;

    Ok(())
}
