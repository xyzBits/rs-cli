mod base64;
mod csv;
mod genpass;

pub use base64::*;
pub use csv::*;
pub use genpass::*;

use crate::cli::csv::CsvOpts;
use crate::cli::genpass::GenPassOpts;
use anyhow::Result;
use clap::Parser;
use std::{path::Path, str::FromStr};

/// 最上层的 command
/// Parser 是 clap 的属性，它是用来解析命令行参数的
/// name 是命令的名称，version 是命令的版本，author 是命令的作者，about 是命令的简介，long_about 是命令的长简介
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    // 子命令，它必须是一个 enum
    // subcommand 是 clap 的属性，它是用来解析子命令的
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/// 多个子命令定义在一个 enum 中
#[derive(Debug, Parser)]
pub enum SubCommand {
    // name 是子命令的名称，about 是子命令的简介
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    // base64 下面还有子命令，因此需要加 subcommand 属性，而不是像 csv 那样直接定义在 Opts 中
    // 如果这里不加 name 属性，会自动将 enum 的名称小写作为 command 的名称
    #[command(subcommand, name = "base64", about = "Encode and decode base64")]
    Base64(Base64SubCommand),
}

/// 校验输入文件是否存在, 如果存在则返回文件的路径，否则返回错误
/// 它是一个函数，返回值是 Result<String, &'static str>
/// 校验的逻辑是：先创建一个 Path 对象，然后调用 exists 方法来判断文件是否存在，如果存在则返回文件的路径，否则返回错误
/// anyhow::Result 是 anyhow 库的类型，它是用来处理错误的，它是用来返回错误的，而不是抛出错误
/// Err 是错误的类型，它是用来返回错误的，而不是抛出错误
///
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
