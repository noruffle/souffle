use super::Database;
use crate::models::user::User;

extern crate dotenv;

use std::env;
use dotenv::dotenv;
use mongodb::sync::Client;

pub trait Connection {
  fn connect() -> Self;
}

impl Connection for Database {
  fn connect() -> Self 
  {
    dotenv().ok();
  
    let uri = match env::var("URI") {
      Ok(value) => {value.to_string()},
      Err(_) => format!("Error loading env variable.")
    };

    let client = Client::with_uri_str(uri).unwrap();
    let database = client.database("database");
    let collection = database.collection::<User>("users");
  
    Self { collection }
  }
}