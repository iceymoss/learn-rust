use axum::http;
use serde::Serialize;
use axum::Json;

use crate::{dto::todo, service}; // 添加 Serialize trait

// 确保 CreateTodoResp 实现 Serialize trait
#[derive(Serialize)] // 添加这一行
pub struct CreateTodoResp {
    pub id: i32,
    pub description: String,
    pub completed: i8,
    pub created_at: i64,
    pub updated_at: i64,
}

// 原函数保持不变
pub async fn create_todo(input: Json<todo::CreateTodoReq>) -> Result<Json<CreateTodoResp>, (http::StatusCode, String)> {
    if input.description.is_empty() {
        return Err((http::StatusCode::BAD_REQUEST, "参数为空".to_string()));
    }

    let resp = service::todo::create_todo(&input.description).await;
    match resp {
        Ok(data) => {
            let res = CreateTodoResp {
                id: data.id,
                description: data.description,
                completed: data.completed as i8,
                created_at: data.created_at.timestamp(),
                updated_at: data.updated_at.timestamp(),
            };
            Ok(Json(res))
        }
        Err(e) => {
            Err((http::StatusCode::INTERNAL_SERVER_ERROR, "创建失败".to_string()))
        }
    }
}