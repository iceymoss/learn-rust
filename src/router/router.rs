use axum::{routing::post, Router};
use axum::routing::{delete, get, put};
use crate::handle::todo;

pub fn todo_routes(r: Router) -> Router {
    r.nest("/api/v1",
                Router::new()
                    .route("/todo/create", post(todo::create_todo))
                    .route("/todo/list", post(todo::list_todo))
                    .route("/todo/:id", get(todo::get_todo))
                    .route("/todo/:id", delete(todo::delete_todo))
                    .route("/todo", put(todo::update_todo))
    )
}