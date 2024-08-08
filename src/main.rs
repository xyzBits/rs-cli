use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    // 解析命令行参数
    let opts = Opts::parse();

    // 根据命令行参数的不同，执行不同的操作
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }

    Ok(())
}
