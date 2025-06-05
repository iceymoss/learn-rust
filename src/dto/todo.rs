use core::str;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTodoReq {
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTodoResp {
    pub id: i32,
    pub description: String,
    pub completed: i8,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListTodoReq {
    pub state: i8,
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Serialize)]
pub struct ListTodoResp {
    pub list: Vec<CreateTodoResp>,
    pub total: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub id: i32,
    pub description: String,
    pub completed: i8,
}

