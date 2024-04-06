extern crate dotenv;

use crate::models::User;

use std::env;
use dotenv::dotenv;
use mongodb::
{
  bson::{doc, extjson::de::Error, oid::ObjectId},
  results::{InsertOneResult, UpdateResult, DeleteResult},
  sync::{Client, Collection}
};

/* pub trait Database {
  fn init() -> Self;
  fn create(&self, ctx: User) -> Result<InsertOneResult, Error>;
  fn get(&self, ctx: &String) -> Result<User, Error>;
  fn get_all(&self) -> Result<Vec<User>, Error>;
  fn update(&self, id: &String, new_user: User) -> Result<UpdateResult, Error>;
  fn delete(&self, id: &String) -> Result<DeleteResult, Error>; 
} */

#[allow(dead_code)]
pub struct Users {
  collection: Collection<User>,
}

impl Users {
  pub fn init() -> Self {
    dotenv().ok();
  
    let uri = match env::var("MONGO_URI") {
      Ok(value) => value.to_string(),
      Err(_) => format!("Error loading env variable.")
    };
  
    let client = Client::with_uri_str(uri).unwrap();
    let database = client.database("Database");
    let collection = database.collection("User");
  
    Self { collection }   
  }
  
  pub fn create(&self, ctx: User) -> Result<InsertOneResult, Error> {
    let new = User {
      id: None,
      name: ctx.name,
      email: ctx.email,
      password: ctx.password
    };

    let user = self.collection.insert_one(new, None).ok()
      .expect("Creating user error");

    Ok(user)
  }
  
  pub fn get(&self, id: &String) -> Result<User, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
      let filter = doc! {"_id": obj_id};
      let user_detail = self
          .collection
          .find_one(filter, None)
          .ok()
          .expect("Error getting user's detail");
      Ok(user_detail.unwrap())
  }

  pub fn get_all(&self) -> Result<Vec<User>, Error> {
    let cursors = self
      .collection
      .find(None, None)
      .ok()
      .expect("Error getting list of users");
    let users = cursors.map(|doc| doc.unwrap()).collect();

    Ok(users)
  }
  
  pub fn update(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let new_doc = doc! {
      "$set":
        {
          "id": new_user.id,
          "name": new_user.name,
          "email": new_user.email,
          "password": new_user.password
        },
    };

    let updated_doc = self
        .collection
        .update_one(filter, new_doc, None)
        .ok()
        .expect("Error updating user");
    Ok(updated_doc)
  }
  
  pub fn delete(&self, id: &String) -> Result<DeleteResult, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let user_detail = self
      .collection
      .delete_one(filter, None)
      .ok()
      .expect("Error deleting user");

    Ok(user_detail)
  }
}
