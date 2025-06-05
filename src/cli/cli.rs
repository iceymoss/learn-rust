use anyhow::Result;
use clap::Parser;
use learn_rust::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {

    // 解析并执行命令
    let cli = Cli::parse();
    cli.command.exec().await?;
    Ok(())
}