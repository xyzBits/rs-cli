use clap::Parser;
use rcli::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // 解析命令行参数
    let opts = Opts::parse();

    // opts 的 sub cmd execute 进行执行
    opts.cmd.execute().await?;

    Ok(())
}
