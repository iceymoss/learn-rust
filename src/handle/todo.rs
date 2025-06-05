use axum::http;
use axum::Json;
use crate::{dto::todo, service};
use axum::extract::Path;

pub async fn create_todo(input: Json<todo::CreateTodoReq>) -> Result<Json<todo::CreateTodoResp>, (http::StatusCode, String)> {
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
            println!("创建失败: {:?}", e);
            Err((http::StatusCode::INTERNAL_SERVER_ERROR, "创建失败".to_string()))
        }
    }
}

pub async fn list_todo(mut input: Json<todo::ListTodoReq>) -> Result<Json<todo::ListTodoResp>, (http::StatusCode, String)> {
    if input.page <= 0 {
        input.page = 1;
    };

    if input.page_size <= 0 || input.page_size > 50 {
        input.page_size = 50;
    };

    let resp = service::todo::list_todo(todo::ListTodoReq{
        state: input.state,
        page: input.page,
        page_size: input.page_size,
    }).await;
    match resp {
        Ok(data) => {
            let mut list: Vec<todo::CreateTodoResp> = vec![];
            for i in &data.list {
                let item = todo::CreateTodoResp {
                    id: i.id,
                    completed: i.completed as i8,
                    description: i.description.clone(),
                    created_at: i.created_at.timestamp(),
                    updated_at: i.updated_at.timestamp(),
                };
                list.push(item);
            }
            let res = todo::ListTodoResp {
                list: list,
                total: data.total,
            };
            Ok(Json(res))
        }
        Err(e) => {
            println!("获取待办事项列表失败: {:?}", e);
            Err((http::StatusCode::INTERNAL_SERVER_ERROR, "获取待办事项列表失败".to_string()))
        }
    }
}

pub async fn get_todo(Path(id): Path<i32>) -> Result<Json<todo::CreateTodoResp>, (http::StatusCode, String)> {
    if id == 0 {
        return Err((http::StatusCode::BAD_REQUEST, "id不能为空".to_string()))
    };
    
    let resp = service::todo::get_todo(id).await;
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
            println!("获取待办事项失败: {:?}", e);
            Err((http::StatusCode::INTERNAL_SERVER_ERROR, "获取待办事项失败".to_string()))
        }
    }
}

pub async fn delete_todo(Path(id): Path<i32>) -> Result<Json<()>, (http::StatusCode, String)> {
    if id == 0 {
        return Err((http::StatusCode::BAD_REQUEST, "id不能为空".to_string()))
    };
    
    let resp = service::todo::delete_todo(id).await;
    match resp {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            println!("移除待办事项失败{:?}", e);
            Err((http::StatusCode::INTERNAL_SERVER_ERROR, "移除待办事项失败".to_string()))
        },
    }
}

pub async fn update_todo(input: Json<todo::UpdateTodo>) -> Result<Json<()>, (http::StatusCode, String)> {
    if input.id == 0 {
        return Err((http::StatusCode::BAD_REQUEST, "id不能为空".to_string()))
    };
    
    if input.description == "" {
        return Err((http::StatusCode::BAD_REQUEST, "待办内容不能为空".to_string()))
    }
    
    let resp = service::todo::update_todo(todo::UpdateTodo {
        description: input.description.clone(),
        id:input.id,
        completed: input.completed,
    }).await;
    match resp {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            println!("移除待办事项失败{:?}", e);
            Err((http::StatusCode::INTERNAL_SERVER_ERROR, "移除待办事项失败".to_string()))
        },
    }
}
