use crate::{database::Users, models::User};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

//GET
//GET ALL
//POST
//UPDATE
//DELETE

#[get("/users")]
pub fn get_all_users(database: &State<Users>) -> Result<Json<Vec<User>>, Status> {
  let users = database.get_all();

  match users {
    Ok(users) => Ok(Json(users)),
    Err(_) => Err(Status::BadRequest),
  }
}

#[post("/user", format = "json", data = "<new_user>")]
pub fn create_user(database: &State<Users>, new_user: Json<User>) -> Result<Json<InsertOneResult>, Status>  {
  let data = User {
    id: None,
    name: new_user.name.to_owned(),
    email: new_user.email.to_owned(),
    password: new_user.password.to_owned(),
  };

  let user_detail = database.create(data);

  match user_detail {
    Ok(user) => Ok(Json(user)),
    Err(_) => Err(Status::BadRequest),
  }
}
