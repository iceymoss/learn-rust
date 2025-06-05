use axum::{
    body::Body, extract::Request, http::StatusCode, response::{Html, IntoResponse}, routing::get, Router
};
use tower_http::services::{ServeDir, ServeFile};
use std::path::PathBuf;
use std::env;
use tower_http::trace::TraceLayer;
use learn_rust::db::mysql;
use dotenvy;
use learn_rust::router::router::todo_routes;

#[tokio::main]
async fn main() {
    // 加载配置
    dotenvy::dotenv().ok(); // 加载 .env 文件

    // 导入日志
    tracing_subscriber::fmt::init();

    // 初始化db
    let _db =  mysql::init().await.expect("db 初始化失败");

    let project_root = env::current_dir().expect("Failed to get current directory");
    
    // 1. 明确指定项目根目录
    let project_root = PathBuf::from(project_root);
    let static_path = project_root.join("static");
    
    // 2. 验证静态文件路径是否存在
    if !static_path.exists() {
        eprintln!("错误: 静态文件目录不存在: {:?}", static_path);
        std::process::exit(1);
    }
    
    println!("项目根目录: {:?}", project_root);
    println!("静态文件目录: {:?}", static_path);
    
    // 3. 配置静态文件服务
    let serve_dir = ServeDir::new(&static_path)
        .not_found_service(ServeFile::new(static_path.join("index.html")));
    
    // 4. 构建路由
    let app = Router::new()
        // API 示例端点
        .route("/api/hello", get(hello_api))
        // .route("/todo/create", post(handle::todo::create_todo))
        // 静态资源服务
        .nest_service("/static", ServeDir::new(&static_path))
        // 路由示例
        .route("/hello", get(hello_page))

        .layer(TraceLayer::new_for_http())
        // 捕获所有未匹配路由并返回 SPA 页面
        .fallback(get(handle_spa_fallback))
        // 回退服务（确保正确处理文件服务）
        .fallback_service(serve_dir.clone());

    let app = todo_routes(app);

    // 5. 启动服务器
    let addr = "127.0.0.1:3000";
    tracing::debug!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("服务器运行在 http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// API 端点示例
async fn hello_api() -> impl IntoResponse {
    (StatusCode::OK, r#"{"message": "Hello from API!"}"#)
}

// 页面路由示例
async fn hello_page() -> impl IntoResponse {
    Html("<h1>Hello Page!</h1><p>This is a route page example</p>")
}

// SPA 回退处理
async fn handle_spa_fallback(_request: Request<Body>) -> impl IntoResponse {
    let project_root = PathBuf::from("/Users/iceymoss/project/learn-rust");
    let index_path = project_root.join("static/index.html");
    
    match tokio::fs::read_to_string(&index_path).await {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            format!("Index file not found at {:?}", index_path)
        ).into_response(),
    }
}