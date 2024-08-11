use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use clap::Parser;
use rcli::{get_content, get_reader, process_csv, process_decode, process_encode, process_genpass, process_http_serve, process_text_key_generate, process_text_sign, process_text_verify, Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSubCommand};
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // 解析命令行参数
    let opts = Opts::parse();

    // 根据命令行参数的不同，执行不同的操作
    match opts.cmd {
        SubCommand::Csv(opts) => {
            // 处理可选参数，为可选参数搞一个默认值
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // opts.format 会调用 debug 的 fmt
                format!("output.{}", opts.format)
            };

            process_csv(&opts.input, output, opts.format)?;
        }

        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }

        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },

        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::KeyGenerate(opts) => {
                let key_map = process_text_key_generate(opts.format)?;
                for (k, v) in key_map {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }

            TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                // opts.key 也是一个文件路径
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;

                // base64 output
                let encode = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encode);
            }

            TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;

                let decoded_sig = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded_sig, opts.format)?;

                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
        },

        SubCommand::Http(cmd) => match cmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        }

        _ => {}
    }

    Ok(())
}
