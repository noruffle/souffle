use crate::services::database::Database;
use crate::models::user::User;

extern crate dotenv;

use mongodb::{
  bson::{
    doc, 
    oid::ObjectId,
    extjson::de::Error,
  }, 
  results::{
    UpdateResult, 
    DeleteResult,
  }
};

pub trait Search {
  fn find_one(&self, id: &String) -> Result<User, Error>;
  fn find_all(&self) -> Result<Vec<User>, Error>;
  fn update(&self, id: &String, new_user: User) -> Result<UpdateResult, Error>;
  fn delete(&self, id: &String) -> Result<DeleteResult, Error>;
}

impl Search for Database {

  fn find_one(&self, id: &String) -> Result<User, Error>
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
  
  fn find_all(&self) -> Result<Vec<User>, Error> 
  {
    let cursors = self
      .collection
      .find(None, None)
      .ok()
      .expect("Error getting list of users");
    let users = cursors.map(|doc| doc.unwrap()).collect();
    Ok(users)
  }
  
  fn update(&self, id: &String, new_user: User) -> Result<UpdateResult, Error>
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
  
  fn delete(&self, id: &String) -> Result<DeleteResult, Error>
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