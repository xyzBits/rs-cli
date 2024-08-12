mod cli;
mod process;
mod utils;

use enum_dispatch::enum_dispatch;
pub use cli::*;
pub use process::*;
pub use utils::*;

/// 每一个 subcommand execute 某种行为
/// 构建出一个 trait ，让每个 subcommand 去实现这个 trait
/// 这个 pub trait 只是给 main 用的
/// 1.75版本之前，在 trait 中要使用 async 不太好，
/// 1.75 之后，可以直接在 trait 中定义 async 函数
#[allow(async_fn_in_trait)]
#[enum_dispatch]// 首先在 trait 上加上宏 enum_dispatch
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;

    fn hello() -> CsvOpts;
}
