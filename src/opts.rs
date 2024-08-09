use anyhow::Result;
use clap::Parser;
use std::{fmt, path::Path, str::FromStr};

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
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
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

    // 可选参数，如果没有传入，那么默认值是 None
    #[arg(short, long, default_value = "output.json")]
    pub output: Option<String>,

    // value_parser 是值解析器，它是一个函数，用于解析参数的值，一般输入的参数是字符串引用
    // 那么解析函数需要实现 字符串引用 和 enum 之间的转换，
    // 所以需要为 enum 实现 FromStr trait
    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    // header 的作用是：是否有 header，如果有 header，那么第一行是 header，如果没有 header，那么第一行是数据
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    // error: unexpected argument 'false' found
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
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

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // pub fn parse<F>(&self) -> Result<F, F::Err>
    // 根据返回值类型推导需要 parse 的类型
    // FromStr::from_str(self) 里面会调用这个方法，最终调用到 from_str 方法
    format.parse()
}

/// 将 OutputFormat 转换为 字符串
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

/// 将字符串转换为 OutputFormat 类型
/// 将 字符串切片 转换为 一个 enum 类型
/// 它是一个函数，返回值是 Result<OutputFormat, Self::Err>
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
