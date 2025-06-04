use axum::{routing::post, Router};
use crate::handle::todo;

pub fn todo_routes(r: Router) -> Router {
    r.nest("/api/v1",
                Router::new()
                    .route("/doto/create", post(todo::create_todo))
    )
}