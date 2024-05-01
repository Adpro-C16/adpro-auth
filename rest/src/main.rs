#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use rocket_cors::AllowedOrigins;
// use shuttle_rocket::ShuttleRocket;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub mod model;
pub mod route;

#[get("/")]
pub fn index() -> &'static str {
    "Heymart C14 - Auth Service"
}

// #[shuttle_runtime::main]
// async fn main(#[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore) -> ShuttleRocket {
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

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
        .mount("/auth", routes![route::auth::login, route::auth::register,])
        .mount(
            "/user",
            routes![
                route::user::get_user,
                route::user::update_balance,
                route::user::topup_balance
            ],
        )
        .mount("/", routes![index]);

    rocket.launch().await?;

    Ok(())
}
