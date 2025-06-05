// 文件: src/todo_service.rs
// 业务逻辑层 - ToDo相关的操作

use crate::{db, entities::todo, dto};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};
use anyhow::{Result, anyhow}; // 统一错误处理

// 创建新任务
pub async fn create_todo(description: &str) -> Result<todo::Model> {
    // 获取数据库连接
    let db = db::mysql::get_connection();
    
    println!("创建任务: '{}'", description);
    
    // 创建活动模型
    let new_todo = todo::ActiveModel {
        description: sea_orm::Set(description.to_string()), // 设置描述
        completed: sea_orm::Set(false), // 默认为未完成
        ..Default::default() // 其他字段使用默认值
    };
    
    // 插入数据库
    match new_todo.insert(db).await {
        Ok(result) => {
            println!("任务创建成功，ID: {}", result.id);
            Ok(result)
        }
        Err(e) => {
            println!("任务创建失败: {}", e);
            Err(anyhow!("数据库错误: {}", e))
        }
    }
}


pub async fn ceate(description: &str) -> Result<todo::Model> {
    // 获取数据库
    let db = db::mysql::get_connection();

    // 构建模型
    let new_todo = todo::ActiveModel {
        description: sea_orm::Set(description.to_string()),
        completed: sea_orm::Set(false),
        ..Default::default()
    };

    // 写入
    let res = new_todo.insert(db).await;
    match res {
        Ok(data) => {
            Ok(data)
        }
        Err(e) => {
            println!("创建待办事项失败{:?}", e);
            Err(anyhow!("创建待办事项失败: {}", e))
        }
    }
}

pub struct TodoList {
    pub list: Vec<todo::Model>,
    pub total: i32,
}

pub async fn list_todo(input: dto::todo::ListTodoReq) -> Result<TodoList> {
    let db = db::mysql::get_connection();
    
    let mut todo_query = todo::Entity::find();
    
    let total_query = todo_query.clone();
    let total = total_query.count(db).await?;
    
    if input.state == 1 {
        // 所有权需要重新给到todo_query
        todo_query = todo_query.filter(todo::Column::Completed.eq(1));
    } else if input.state == 2 {
        todo_query = todo_query.filter(todo::Column::Completed.eq(0));
    }

    // 计算偏移量, 类型转换
    let offset: u64 = ((input.page - 1) * input.page_size) as u64;
    let query = todo_query.offset(offset).limit(input.page_size as u64);
    
    let res = query.all(db).await;
    match res { 
        Ok(data) => {
            let ans = TodoList {
                list: data,
                total: total as i32,
            };
            Ok(ans)
        }
        Err(e) => {
            println!("获取待办事项列表失败{:?}", e);
            Err(anyhow!("获取待办事项列表失败{:?}", e))
        }
    }
}

pub async fn get_todo(id: i32) -> Result<todo::Model> {
    let db = db::mysql::get_connection();
    let res = todo::Entity::find_by_id(id).one(db).await;
    match res {
        Ok(data) => {
            match data { 
                Some(d) => {
                    Ok(d)
                }
                None => {
                    Ok(todo::Model{..data.unwrap()})
                }
            }
        }
        Err(e) => {
            println!("获取待办事项失败{:?}", e);
            Err(anyhow!("获取待办事项失败{:?}", e))
        }
    }
}

pub async fn delete_todo(id: i32) -> Result<()> {
    let db = db::mysql::get_connection();
    let _todo = todo::Entity::find_by_id(id).one(db).await.unwrap().ok_or(DbErr::RecordNotFound(format!("该待办事项 {} 不存在", id)))?;
    
    let delete_result = todo::Entity::delete_by_id(id).exec(db).await;
    match delete_result {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            println!("移除待办事项失败{:?}", e);
            Err(anyhow!("移除待办事项失败{:?}", e))
        }
    }
}

pub async fn update_todo(input: dto::todo::UpdateTodo) -> Result<()> {
    let db = db::mysql::get_connection();
    let todo_temp = todo::Entity::find_by_id(input.id).one(db).await.unwrap().ok_or(DbErr::RecordNotFound(format!("该待办事项 {} 不存在", input.id)))?;
    let mut completed: bool = false;
    if input.completed == 1 {
        completed = true;
    }
    let data = todo::ActiveModel {
        id: sea_orm::Set(todo_temp.id),
        description: sea_orm::Set(input.description),
        completed: sea_orm::Set(completed),
        created_at: sea_orm::Set(todo_temp.created_at),
        ..Default::default() // 其他字段使用默认值
    };
    let resp = todo::Entity::update(data).exec(db).await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("更新待办事项失败{:?}", e))
    }
}




// // 获取所有任务，按ID排序
// pub async fn get_all_todos() -> Result<Vec<todo::Model>> {
//     // 获取数据库连接
//     let db = db::get_connection();
//     println!("查询所有任务...");
    
//     // 查询所有记录
//     match todo::Entity::find()
//         .order_by_asc(todo::Column::Id) // 按ID升序排序
//         .all(db)
//         .await
//     {
//         Ok(todos) => {
//             println!("找到 {} 条任务", todos.len());
//             Ok(todos)
//         }
//         Err(e) => {
//             println!("查询失败: {}", e);
//             Err(anyhow!("数据库错误: {}", e))
//         }
//     }
// }

// // 通过ID查询单个任务
// pub async fn get_todo_by_id(id: i32) -> Result<Option<todo::Model>> {
//     let db = db::get_connection();
//     println!("查询任务ID: {}...", id);
    
//     // 使用find_by_id方法
//     match todo::Entity::find_by_id(id)
//         .one(db)
//         .await
//     {
//         Ok(Some(todo)) => {
//             println!("找到任务ID: {}", id);
//             Ok(Some(todo))
//         }
//         Ok(None) => {
//             println!("未找到任务ID: {}", id);
//             Ok(None)
//         }
//         Err(e) => {
//             println!("查询失败: {}", e);
//             Err(anyhow!("数据库错误: {}", e))
//         }
//     }
// }

// // 更新任务（可以更新描述和状态）
// pub async fn update_todo(
//     id: i32, 
//     description: Option<&str>, // 可选的描述更新
//     completed: Option<bool>   // 可选的状态更新
// ) -> Result<todo::Model> {
//     let db = db::get_connection();
//     println!("更新任务ID: {}...", id);
    
//     // 1. 先查找任务
//     let existing = match todo::Entity::find_by_id(id)
//         .one(db)
//         .await?
//     {
//         Some(todo) => todo,
//         None => return Err(anyhow!("任务ID {} 不存在", id)),
//     };
    
//     // 2. 转换为活动模型以便更新
//     let mut active_model: todo::ActiveModel = existing.into();
    
//     // 3. 根据需要更新字段
//     if let Some(desc) = description {
//         println!("更新描述为: '{}'", desc);
//         active_model.description = sea_orm::Set(desc.to_string());
//     }
    
//     if let Some(comp) = completed {
//         println!("更新状态为: {}", if comp { "已完成" } else { "未完成" });
//         active_model.completed = sea_orm::Set(comp);
//     }
    
//     // 4. 保存更新（会触发updated_at更新）
//     match active_model.save(db).await {
//         Ok(saved) => {
//             // 将活动模型转回普通模型
//             match saved.try_into_model() {
//                 Ok(model) => {
//                     println!("任务更新成功!");
//                     Ok(model)
//                 }
//                 Err(e) => {
//                     println!("模型转换失败: {}", e);
//                     Err(anyhow!("内部错误: {}", e))
//                 }
//             }
//         }
//         Err(e) => {
//             println!("更新失败: {}", e);
//             Err(anyhow!("数据库错误: {}", e))
//         }
//     }
// }

// // 删除任务
// pub async fn delete_todo(id: i32) -> Result<u64> {
//     let db = db::get_connection();
//     println!("删除任务ID: {}...", id);
    
//     // 使用delete_by_id方法
//     match todo::Entity::delete_by_id(id)
//         .exec(db)
//         .await
//     {
//         Ok(result) => {
//             println!("删除成功! 影响行数: {}", result.rows_affected);
//             Ok(result.rows_affected)
//         }
//         Err(e) => {
//             println!("删除失败: {}", e);
//             Err(anyhow!("数据库错误: {}", e))
//         }
//     }
// }