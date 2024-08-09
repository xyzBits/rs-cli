use crate::opts::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

/// deserialize 是反序列化，它是用来将 json 格式的数据转换为 struct 的
/// serialize 是序列化，它是用来将 struct 转换为 json 格式的
/// serde 是序列化和反序列化的库，它是用来处理 json 格式的
/// PascalCase 是大驼峰命名法，它是用来命名 struct 的属性的
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,

    // rename 是用来重命名属性的，它是用来将属性的名称进行更改的
    // 它是用来将属性的名称进行更改的，它是用来将属性的名称进行更改的
    #[serde(rename = "DOB")]
    dob: String,

    nationality: String,

    // rename 是用来重命名属性的，它是用来将属性的名称进行更改的
    // 它是用来将属性的名称进行更改的，它是用来将属性的名称进行更改的
    #[serde(rename = "Kit Number")]
    kit: u8,
}
// 使用 csv 库来读取 csv 文件，使用 serde_json 库来将数据转换为 json 格式
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    // Reader::from_path 是用来创建一个 Reader 对象的，它是用来读取 csv 文件的
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);

    let headers = reader.headers()?.clone();

    // 统一的转为 json 格式，而不是和 struct 进行绑定
    for result in reader.records() {
        let record = result?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }

    // to_string_pretty 是用来将 struct 转换为 json 格式的，它是用来将 struct 转换为 json 格式的

    let content = match format {
        // 将 数据结构 ret 序列化为 json 格式
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    // fs::write 是用来将数据写入到文件中的，它是用来将数据写入到文件中的
    fs::write(output, content)?;

    Ok(())
}
