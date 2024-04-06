#[get("/")]
pub async fn get_home() -> &'static str {
  "Hello, world!"
}

use rocket::response::content::RawJson;

#[get("/api/data")]
pub fn get_data() -> RawJson<&'static str> {
    RawJson(r#"{"message": "Hello from Rocket!"}"#)
}