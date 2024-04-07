use rocket::response::content::RawJson;

#[get("/")]
pub async fn get_home() -> &'static str {
  "Hello, world!"
}

#[get("/api/data")]
pub fn get_data() -> RawJson<&'static str> {
    RawJson(r#"{"message": "Hello from Rocket!"}"#)
}