use crate::{database::Users, models::User};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data = "<new_user>")]
pub fn create_user(
  database: &State<Users>,
  new_user: Json<User>
) -> Result<Json<InsertOneResult>, Status> {
  let data = User {
    id: None,
    name: new_user.name.to_owned(),
    email: new_user.email.to_owned(),
    password: new_user.password.to_owned(),
  };

  let user_details = database.create(data);

  match user_details {
    Ok(user) => Ok(Json(user)),
    Err(_) => Err(Status::BadRequest),
  }
}