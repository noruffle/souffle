#[macro_use] extern crate rocket;

mod services;
mod database;
mod models;
mod controllers;

use database::Users;

#[launch]
fn rocket() -> _ {

  let connection_of_database = Users::init();

  rocket::build()
    .manage(connection_of_database)
    .mount(
      "/", 
      routes![
        // default
        controllers::default::get_data, 
        controllers::default::get_home,

        // users
        controllers::users::create_user,
        controllers::users::get_all_users,
      ]   
    )
}