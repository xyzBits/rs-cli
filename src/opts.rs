use anyhow::Result;
use clap::Parser;
use std::path::Path;

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
}

/// 单个子命令的参数定义在一个 struct 中
/// 参数的类型是 clap 的 arg 宏的参数
/// short 是短参数，long 是长参数
/// value_parser 是值解析器，它是一个函数，用于解析参数的值，其实是用来校验输入参数的值
/// default_value 是默认值，它是一个字符串，用于设置参数的默认值
/// default_value_t 是默认值的类型，它是一个类型，用于设置参数的默认值  
/// 两者的区别是：default_value 是字符串，要使用 into from 进行转换，而 default_value_t 就是所要求的类型，不需要进行转换
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    // header 的作用是：是否有 header，如果有 header，那么第一行是 header，如果没有 header，那么第一行是数据
    #[arg(long, default_value_t = true)]
    pub header: bool,
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
