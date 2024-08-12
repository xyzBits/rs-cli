use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_http_serve, process_text_key_generate, process_text_sign, process_text_verify,
    Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSubCommand,
};
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // 解析命令行参数
    let opts = Opts::parse();
    opts.cmd.execute().await?;

    Ok(())
}
