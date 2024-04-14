use crate::services::database::search::Search;
use crate::services::database::Database;
use crate::models::user::User;

use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};

#[get("/user/<query>")]
pub fn find_one(
  database: &State<Database>, 
  query: String
) -> Result<Json<User>, Status>
{
  let props = query;
  
  if props.is_empty() { return Err( Status::BadRequest )};

  match database.find_one(&props) {
    Ok(user) => Ok(Json(user)),
    Err(_) => Err(
      Status::BadRequest
    )
  }
}

#[get("/users")]
pub fn find_all(
  database: &State<Database>
) -> Result<Json<Vec<User>>, Status> 
{
  match database.find_all() {
    Ok(list_of_users) => Ok(Json(list_of_users)),
    Err(_) => Err(
      Status::BadRequest
    )
  }
}

#[put("/user/<query>", data = "<ctx>")]
pub fn update(
  database: &State<Database>,
  query: String,
  ctx: Json<User>
) -> Result<Json<User>, Status>
{
  let props = query;

  if props.is_empty() { return Err( Status::BadRequest )};

  let data = User {
    id: Some(ObjectId::parse_str(&props).unwrap()),
    name: ctx.name.to_owned(),
    email: ctx.email.to_owned(),
    password: ctx.password.to_owned(),
  };

  match database.update(&props, data) {
    Ok(upd) => {
      if upd.matched_count == 1 {
        return match database.find_one(&props) {
          Ok(user) => Ok(Json(user)),
          Err(_) => Err( Status::BadRequest )
        };
      } else {
        return Err( Status:: NotFound );
      }
    },
    Err(_) => Err( Status::InternalServerError ),
  }
}

#[delete("/user/<query>")]
pub fn delete(
  database: &State<Database>, 
  query: String
) -> Result<Json<&str>, Status>
{
  let props = query;
  
  if props.is_empty() { return Err(Status::BadRequest) };

  match database.delete(&props) {
    Ok(res) => if res.deleted_count == 1 {
      return Ok(Json("User deleted"))
    } else {
      return Err( Status::NotFound );
    },
    Err(_) => Err( Status::InternalServerError ),
  }
}