use core::str;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTodoReq {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoResp {
    pub id: i32,
    pub description: String,
    pub completed: i8,
    pub created_at: i64,
    pub updated_at: i64,
}


