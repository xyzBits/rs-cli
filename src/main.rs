use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    // 解析命令行参数
    let opts = Opts::parse();

    // 根据命令行参数的不同，执行不同的操作
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // opts.format 会调用 debug 的 fmt
                format!("output.{}", opts.format)
            };

            process_csv(&opts.input, output, opts.format)?;
        }
        _ => {}
    }

    Ok(())
}
