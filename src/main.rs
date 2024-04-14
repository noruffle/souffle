#[macro_use] extern crate rocket;

mod services;
mod models;
mod controllers;

#[launch]
fn rocket() -> _ 
{
  use services::database::{Database, Connection};

  let connection_of_database = Database::connect();

  rocket::build()
    .manage(connection_of_database)
    .mount(
      "/", 
      routes![
        // default
        controllers::default::get_data, 
        controllers::default::get_home,

        // Authorisation

        // Users search
        controllers::user::search::update,
        controllers::user::search::delete,
        controllers::user::search::find_all,
        controllers::user::search::find_one,
      ]   
    )
}