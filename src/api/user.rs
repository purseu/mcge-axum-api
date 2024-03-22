use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::db::User;

use super::ApiError;


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
) -> Result<Json<AuthBody>, ApiError> {
    let wx_user = wx_login(payload.code).await?;

    let user = sqlx::query_as::<_, User>("select * from users where openid = ?")
    .bind(&wx_user.openid)
    .fetch_one(&pool)
    .await;

    let user: User = match user {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            let res = sqlx::query("insert into users (openid, session_key) values (?, ?)")
            .bind(&wx_user.openid)
            .bind(&wx_user.session_key)
            .execute(&pool)
            .await;

            sqlx::query_as::<_, User>("select * from users where openid = ?")
            .bind(&wx_user.openid)
            .fetch_one(&pool)
            .await?
        },
        Err(e) => return Err(ApiError::Internal)
    };

    Ok(Json(()))
}

#[derive(Deserialize, Default)]
struct WxUser {
    pub openid: String,
    pub session_key: String,
}

async fn wx_login(code: String) -> Result<WxUser, ApiError> {
    Ok(WxUser::default())
}



