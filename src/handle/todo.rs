use std::f32::consts::E;

use crate::dto::todo;
use crate::service;
use axum::Json;
use axum::http;

pub async fn create_todo(input: axum::Json<todo::CreateTodoReq>) -> Result<axum::Json<todo::CreateTodoResp>, (axum::http::StatusCode, String)> {
    // 获取参数
    if input.description.is_empty() {
        return Err((http::StatusCode::BAD_REQUEST, "参数为空".to_string()));
    }

    let resp = service::todo::create_todo(&input.description).await;
    match resp {
        Ok(data) => {
            let res = todo::CreateTodoResp {
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