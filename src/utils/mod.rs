mod custom_extractors;
pub mod middlewares;

use crate::domain::ctx::Ctx;
use crate::errors::{ApiError, Result, BaseError};
pub use custom_extractors::json_extractor::JsonExtractor;
pub use custom_extractors::path_extractor::PathExtractor;
use std::process::Output;
use tokio::process::Command;

pub async fn sudo_exec(ctx: &Ctx, cmd: Vec<&str>) -> Result<Output> {
    Command::new("sudo")
        .args(cmd.clone())
        .output()
        .await
        .map_err(|_err| BaseError::Execution { source: _err.to_string() })
}
