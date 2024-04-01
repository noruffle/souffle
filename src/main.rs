#[macro_use] extern crate rocket;

mod controllers;
mod database;
mod models;
mod routes;

use routes::*;

#[launch]
fn rocket() -> _ {
  rocket::build()
  .mount("/", routes![get_home])
}