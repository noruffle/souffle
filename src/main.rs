#[macro_use] extern crate rocket;

mod services;
mod database;
mod models;
mod routes;

use database::Users;
use routes::*;

use rocket::response::content::RawJson;

#[get("/api/data")]
fn get_data() -> RawJson<&'static str> {
    RawJson(r#"{"message": "Hello from Rocket!"}"#)
}

#[launch]
fn rocket() -> _ {

  let connection_of_database = Users::init();

  rocket::build()
  .manage(connection_of_database)
  .mount(
    "/", 
    routes![
      get_home, 
      get_data,
      create_user,
    ]
  )
}