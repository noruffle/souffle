#[macro_use] extern crate rocket;

mod database;
mod models;
mod controllers;

use database::Service;

#[launch]
fn rocket() -> _ {

  let connection_of_database = Service::init();

  rocket::build()
    .manage(connection_of_database)
    .mount(
      "/", 
      routes![
        // default
        controllers::default::get_data, 
        controllers::default::get_home,

        // users
        controllers::users::new,
        controllers::users::update,
        controllers::users::delete,
        controllers::users::find_all,
        controllers::users::find_one,
      ]   
    )
}