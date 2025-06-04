// 文件: src/entities.rs
// 定义数据库表对应的模型

use axum::async_trait;
use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;

// 定义ToDo表的模型
// 每行注释说明对应数据库字段
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "todo")] // 对应数据库中的todo表
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, // 主键ID，自动递增
    
    #[sea_orm(column_type = "String(Some(255))")] // 对应VARCHAR(255)
    pub description: String, // 任务描述
    
    #[sea_orm(column_type = "TinyInteger")]
    pub completed: bool, // 完成状态，虽然数据库是TINYINT(1)，我们使用bool
    
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: DateTime<Utc>, // 创建时间
    
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: DateTime<Utc>, // 更新时间
}

// 定义表关系（本表无关联关系）
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


// 实现 ActiveModel 的行为（自动处理时间戳等）
#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    // 插入新记录前的钩子（例如自动设置创建时间）
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            // 如果是插入操作，设置 created_at 为当前时间
            self.created_at = Set(chrono::Utc::now().into());
        }
        Ok(self)
    }
}