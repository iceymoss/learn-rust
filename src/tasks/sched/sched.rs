use anyhow::{Error, Result};
use tokio_cron_scheduler::{Job, JobScheduler};
use std::sync::Arc;

// 定义任务类型：一个返回异步任务的闭包
type ScheduledTask = dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync;

// 简化版调度器
pub struct SimpleScheduler {
    jobs: Vec<(String, Arc<ScheduledTask>)>, // 存储 (cron表达式, 异步任务)
}

impl SimpleScheduler {
    // 创建新调度器
    pub fn new() -> Self {
        SimpleScheduler { jobs: vec![] }
    }

    // 添加任务（参数直接接受 async 块）
    pub fn add_task(&mut self, cron: &str, task: impl Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync + 'static) {
        let task = Arc::new(task);
        self.jobs.push((cron.to_string(), task));
    }

    // 启动调度器
    pub async fn run(self) -> Result<JobScheduler, Error> {
        let sched = JobScheduler::new().await?;

        for (cron, task) in self.jobs {
            let job = Job::new_async(&cron, move |_uuid, _lock| {
                let task = task.clone();
                Box::pin(async move {
                    task().await; //执行方法
                })
            })?;
            sched.add(job).await?;
        }

        sched.start().await?;
        Ok(sched)
    }
}