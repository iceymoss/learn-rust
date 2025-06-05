pub mod ping;

use clap::Args;
use anyhow::Result;

use crate::cli::ping::ping::ping_target;

#[derive(Args)]
pub struct PingCommand {
    /// 目标地址（域名或IP）
    #[arg(
        required = true,
        short = 'H',       // 自定义短选项为 -H
        long = "host"      // 自定义长选项为 --host
    )]
    pub target: String,

    /// 设置超时时间（单位：毫秒）
    #[arg(
        short = 'T',       // 短选项改为 -T
        long = "timeout",  // 长选项保持有意义
        default_value_t = 3000,  // 修改默认值
        value_parser = parse_timeout  // 添加自定义校验
    )]
    pub timeout: u64,

    /// 设置探测次数
    #[arg(
        short = 'n',       // 使用 -n 代替默认的 -c
        long = "number",   // 长选项更明确
        default_value_t = 20,
        value_parser = clap::value_parser!(u32).range(1..=20)  // 限制范围
    )]
    pub count: u32,
}

// 自定义校验函数
fn parse_timeout(s: &str) -> Result<u64, String> {
    let timeout = s.parse().map_err(|_| "必须输入数字".to_string())?;
    if timeout < 100 {
        Err("超时时间不能小于100ms".to_string())
    } else {
        Ok(timeout)
    }
}

impl PingCommand {
    pub async fn run(&self) -> Result<()> {
        // 参数校验
        if self.count == 0 {
            anyhow::bail!("请求次数必须大于0");
        }

        // 调用核心逻辑
        ping_target(
            &self.target,
            self.timeout,
            self.count
        ).await
    }
}