use rocket::get;
// use rocket::response::content::Json;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use std::str::FromStr;

use crate::establish_connection;
use crate::{solana::subscribe_to_programlogs};
// use crate::models::Stream;

#[get("/")]
pub fn index() -> &'static str 
{
    println!("Accessing program logs now....");
    // subscribe_to_programlogs();
    "Accessing program logs now...."
}

// #[get("/<pubkey>")]
// pub fn get_all_stream(pubkey: &str) -> Json<Value> {
//     let pubkey_string = String::from_str(pubkey).unwrap();
//     let conn = establish_connection();
//     Json(
//         json!({"status": "success", "something": "yesss"}),
//     )
// }


// #[get("/sth")]
// pub fn get_all_stream2()  -> &'static str {
//     println!("got into get_all_stream2 method");
//     // let pubkey_string = String::from_str(pubkey).unwrap();
//     // let conn = establish_connection();
//     subscribe_to_programlogs();
//     "hellooo get_all_stream2"
// }