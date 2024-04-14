use mongodb::bson::doc;
use rocket::{http::Status, serde::json::Json, State};
use mongodb::results::InsertOneResult;

use crate::services::database::authorization::Authorization;
use crate::services::database::Database;
use crate::models::user::User;

//Login to the account
#[post("/lifecycle/signin", format = "json", data = "<ctx>")]
pub fn sign_in(database: &State<Database>, ctx: Json<User>) -> Result<Json<InsertOneResult>, Status>
{
  
  
}

