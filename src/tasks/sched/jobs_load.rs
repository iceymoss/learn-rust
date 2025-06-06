use anyhow::Result;
use crate::sched::sched::SimpleScheduler;
use learn_rust::tasks::jobs::time;

pub async fn jobs_load() -> Result<()> {
    let mut scheduler = SimpleScheduler::new();
    
    // 添加任务（需匹配类型）
    scheduler.add_task("0 * * * * *", time::time::say_time);

    // 启动调度器并处理错误
    let _sched = scheduler.run().await?;

    // 保持主线程运行
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}
