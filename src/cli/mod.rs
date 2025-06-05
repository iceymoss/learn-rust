use clap::{command, Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "icey-cli")]
#[command(about = "cli", version = "0.1.0")]
pub struct Cli {

    #[command(subcommand)]
    pub command: Commands,

}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "test", about = "测试命令")]
    Hello(test::Test),

    #[command(name = "ping", about = "ICMP连通性测试")]
    Ping(ping::PingCommand),
}

impl Commands {
    pub async fn exec(&self) -> Result<()> {
        match self {
            Commands::Hello(cmd) => {
                cmd.run().await
            }
            Commands::Ping(cmd) => {
                cmd.run().await
            }
        }
    }
}

pub mod test;
pub mod ping;
