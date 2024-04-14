use super::Database;

pub trait Authorization {
  fn sign_up();
  fn sign_in();
}

impl Authorization for Database {
  fn sign_up() 
  {
      
  }

  fn sign_in() 
  {
    
  }
}