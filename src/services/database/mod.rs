pub mod authorization;
pub mod connection;
pub mod search;

use mongodb::sync::Collection;

use crate::models::user::User;

pub use connection::Connection;

pub struct Database {
  pub collection: Collection<User>,
}