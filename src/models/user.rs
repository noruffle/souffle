use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User 
{
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub name: String,
  pub email: String,
  pub password: String
}

impl User {
  pub fn new(id: Option<ObjectId>, name: String, email: String, password: String) -> User
  {
    User { id, name, email, password }
  } 
}