// 自定义任务函数
mod sched;
use std::fmt::Error;
use crate::sched::jobs_load::jobs_load;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let _j = jobs_load().await;
    Ok(())
}
