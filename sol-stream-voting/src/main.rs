#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;
mod solana;
use rocket::routes;
use solana::subscribe_to_program;


use crate::routes::get_all_stream;
use crate::routes::index;
use crate::solana::get_accounts_and_update;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use rocket::config;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_accounts_and_update();
    
    // subscribe_to_program();

    let cors = rocket_cors::CorsOptions::default().to_cors()?;


    rocket::build()
        .mount("/", routes![index])
        .attach(cors)
        .launch()
        .await?;

    // let cfg = rocket::config::Config::build(rocket::config::Environment::Development)
    // .address("127.0.0.1")
    // .port("4000")   
    // .extra("template_dir",  "web/templates")
    // .unwrap();

    // rocket::custom(cfg)
    //     .mount("/", routes![index, get_all_stream])
    //     .attach(rocket_contrib::Template::fairing())
    //     .launch();

    Ok(())
}
