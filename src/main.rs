use dotenvy::dotenv;
// use rocket_cors::AllowedOrigins;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::error::Error;

#[macro_use]
extern crate rocket;

pub mod guard;
pub mod model;
pub mod route;
pub mod typing;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    // let allowed_origins = AllowedOrigins::some_exact(&["https://www.heymart-c14.com"]);

    // let cors = rocket_cors::CorsOptions {
    //     allowed_origins,
    //     ..Default::default()
    // }
    // .to_cors()?;

    let _rocket = rocket::build()
        .manage(pool)
        // .attach(cors)
        .mount(
            "/auth",
            routes![route::login, route::register, route::authorize],
        )
        .mount("/", routes![route::get_user])
        .launch()
        .await?;

    Ok(())
}
