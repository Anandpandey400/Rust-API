use crate::db::DbPool;
use crate::models::request::{CreateUserRequest, Response, User};
use axum::{Json, extract::State};
use sqlx::Row;
use sqlx::query;

//Add InsertUser
pub async fn add_user(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<Response> {
    let result = query(
        r#"
        Exec InsertUser @name = @p1, @age = @p2, @email = @p3
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.age)
    .bind(&payload.email)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Json(Response {
            message: "User added successfully".to_string(),
            status: "Success",
            code: 201,
        }),
        Err(err) => {
            eprintln!("DB Error: {:?}", err);
            Json(Response {
                message: "Failed to add user".to_string(),
                status: "Error",
                code: 500,
            })
        }
    }
}
// Get all users
pub async fn get_all_users(State(pool): State<DbPool>) -> Json<Vec<User>> {
    let rows = sqlx::query(
        r#"
               EXEC GetAllUsers
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch users");

    let users = rows
        .into_iter()
        .map(|row| User {
            id: row.get("id"),
            name: row.get("name"),
            age: row.get("age"),
            email: row.get("email"),
        })
        .collect();

    Json(users)
}
