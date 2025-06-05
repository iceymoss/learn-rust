pub mod test;

use clap::Args;
use crate::cli::test::test as other_test;

// 参数
#[derive(Args)]
pub struct Test {
    pub say: String,
}

impl Test {
    pub async fn run(&self) -> anyhow::Result<()> {
        other_test::test(&self.say).await
    }
}