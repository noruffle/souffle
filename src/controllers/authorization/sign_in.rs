//Login to the account
#[post("/lifecycle/signin", format = "json", data = "<ctx>")]
pub fn sign_in(database: &State<Database>, ctx: Json<User>) -> Result<Json<InsertOneResult>, Status>
{
  let data = User {
    id: None,
    name: ctx.name.to_owned(),
    email: ctx.email.to_owned(),
    password: ctx.password.to_owned(),
  };

  
}


use rocket::{http::Status, serde::json::Json, State};
use mongodb::results::InsertOneResult;

use crate::services::database::authorization::Authorization;
use crate::services::database::Database;
use crate::models::user::User;