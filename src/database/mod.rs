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

pub struct Service {
  collection: Collection<User>,
}

impl Service {
  pub fn connect() -> Service 
  {
    dotenv().ok();
  
    let uri = match env::var("URI") {
      Ok(value) => {value.to_string()},
      Err(_) => format!("Error loading env variable.")
    };
    let client = Client::with_uri_str(uri).unwrap();
    let database = client.database("database");
    let collection = database.collection("users");
  
    Service { collection }   
  }
  
  pub fn create(&self, new_user: User) -> Result<InsertOneResult, Error>
  {
    let new_user = User::new(None, new_user.name, new_user.email, new_user.password);

    let user = self
      .collection
      .insert_one(new_user, None)
      .ok()
      .expect("Error creating user");

    Ok(user)
  }
  
  pub fn get(&self, id: &String) -> Result<User, Error>
  {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let user_detail = self
      .collection
      .find_one(filter, None)
      .ok()
      .expect("Error getting user's detail");
    Ok(user_detail.unwrap())
  }

  pub fn get_all(&self) -> Result<Vec<User>, Error> 
  {
    let cursors = self
      .collection
      .find(None, None)
      .ok()
      .expect("Error getting list of users");
    let users = cursors.map(|doc| doc.unwrap()).collect();

    Ok(users)
  }
  
  pub fn update(&self, id: &String, new_user: User) -> Result<UpdateResult, Error>
  {
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
  
  pub fn delete(&self, id: &String) -> Result<DeleteResult, Error>
  {
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