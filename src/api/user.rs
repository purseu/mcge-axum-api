use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};


#[derive(Deserialize)]
pub struct LoginPayload {
    code: String
}

#[derive(Serialize)]
pub struct  AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self { access_token, token_type: "Bearer".to_string()}
    }
}

pub async fn login(
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<LoginPayload>
) -> Result<Json<()>, String> {
    todo!()
}


