use crate::{database::Service, models::User};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", format = "json", data = "<ctx>")]
pub fn new(database: &State<Service>, ctx: Json<User>) -> Result<Json<InsertOneResult>, Status>
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

#[get("/user/<query>")]
pub fn find_one(database: &State<Service>, query: String) -> Result<Json<User>, Status>
{
  let props = query;
  
  if props.is_empty() { return Err( Status::BadRequest )};

  match database.get(&props) {
    Ok(user) => Ok(Json(user)),
    Err(_) => Err(
      Status::BadRequest
    )
  }
}

#[get("/users")]
pub fn find_all(database: &State<Service>) -> Result<Json<Vec<User>>, Status> 
{
  match database.get_all() {
    Ok(list_of_users) => Ok(Json(list_of_users)),
    Err(_) => Err(
      Status::BadRequest
    )
  }
}

#[put("/user/<query>", data = "<ctx>")]
pub fn update(database: &State<Service>, query: String, ctx: Json<User>) -> Result<Json<User>, Status>
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
        return match database.get(&props) {
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
pub fn delete(database: &State<Service>, query: String) -> Result<Json<&str>, Status>
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