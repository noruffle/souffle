use argon2::password_hash::Salt;
use argon2::*;
use mongodb::bson::{doc, oid::ObjectId};
use rocket::{serde::json::Json, State};

use crate::models::user::User;

use super::Database;

pub trait Authorization {
  fn sign_up(db: &State<Connection>, username: String, email: String, password: String) -> Result<Json<User>>;
  async fn sign_in(&self, database: &State<Database>, ctx: Json<User>);
}

impl Authorization for Database {
  fn sign_up(db: &State<Database>, name: String, email: String, password: String) -> Result<Json<User>> {
    let argon2 = Argon2::default();
    let salt = Salt::from_b64("somesalt").unwrap();
    
    let res = argon2.hash_password_customized(password, None, None, Params::default(), salt).unwrap();

    let user = User {
        id: Some(ObjectId::new()),
        name,
        password: res,
        email,
    };

    db.collection.insert_one(user, None);

    Ok(Json(user))
  }

  async fn sign_in(&self, database: &State<Database>, ctx: Json<User>) //-> Result<Json<User>, Error>
  {
    let user = db.collection("users").find_one(doc! { "username": username }, None)?;

    match user {
        Some(user) => {
            let password_matches = argon2::argon2id::verify(&user["password_hash"], password.as_bytes())?;

            if password_matches {
                Ok(Json(user))
            } else {
                Err("Incorrect password".to_string())
            }
        }
        None => Err("User not found".to_string()),
    }
  }
}