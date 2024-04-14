// Register an account
#[post("/lifecycle/signup", format = "json", data = "<ctx>")]
pub fn sign_up(database: &State<Database>, ctx: Json<User>) -> Result<Json<InsertOneResult>, Status>
{
  let data = User {
    id: None,
    name: ctx.name.to_owned(),
    email: ctx.email.to_owned(),
    password: ctx.password.to_owned(),
  };

  match database.create(data) {
    Ok(user) => Ok(Json(user)),
    Err(_) => Err(
      Status::BadRequest
    )
  }
}

use rocket::{http::Status, serde::json::Json, State};
use mongodb::results::InsertOneResult;

use crate::services::database::authorization::Authorization;
use crate::services::database::Database;
use crate::models::user::User;