// 文件: src/db.rs
// 数据库连接池管理

use sea_orm::{Database, DatabaseConnection, DbErr, RuntimeErr};
use std::env;
use once_cell::sync::OnceCell; // 一次性初始化的全局变量

// 全局静态变量 - 数据库连接池
// OnceCell 让我们可以初始化一次并在全局访问
static DB_POOL: OnceCell<DatabaseConnection> = OnceCell::new();

// 初始化数据库连接池
pub async fn init() -> Result<(), DbErr> {
    // 从环境变量获取数据库URL
    let database_url = env::var("DATABASE_URL")
        .expect("错误：未设置 DATABASE_URL 环境变量！请检查.env文件");
    
    // 创建数据库连接
    let db = Database::connect(&database_url).await?;
    
    // 将连接存入全局静态变量
    DB_POOL.set(db)

        .map_err(|_| DbErr::Conn(RuntimeErr::Internal("错误：数据库连接池已初始化".to_string())))?;
    
    println!("数据库连接池初始化成功！");
    Ok(())
}

// 获取全局数据库连接
pub fn get_connection() -> &'static DatabaseConnection {
    // 如果get()返回None，表示连接池未初始化，直接panic（程序无法继续）
    DB_POOL.get().expect("错误：数据库连接池未初始化！")
}