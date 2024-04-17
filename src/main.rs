#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use shuttle_rocket::ShuttleRocket;
use sqlx::postgres::PgPoolOptions;
// use rocket_cors::AllowedOrigins;
// use std::env;

pub mod guard;
pub mod model;
pub mod route;
pub mod typing;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore) -> ShuttleRocket {
    dotenv().ok();
    // let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let url = secrets.get("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();

    // let allowed_origins = AllowedOrigins::some_exact(&["https://www.heymart-c14.com"]);

    // let cors = rocket_cors::CorsOptions {
    //     allowed_origins,
    //     ..Default::default()
    // }
    // .to_cors()?;

    let rocket = rocket::build()
        .manage(pool)
        // .attach(cors)
        .mount(
            "/auth",
            routes![route::login, route::register, route::authorize],
        )
        .mount("/", routes![route::get_user]);

    Ok(rocket.into())
}
