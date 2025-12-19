use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub age: i32,
    pub email: String,
}

#[derive(Serialize)]
pub struct Response {
    pub message: String,
    pub status: &'static str,
    pub code: u16,
}
#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub age: i32,
}
