use super::verify_input_file;
use clap::Parser;
use std::fmt;
use std::str::FromStr;

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

fn parse_format(format: &str) -> anyhow::Result<OutputFormat, anyhow::Error> {
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

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
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
