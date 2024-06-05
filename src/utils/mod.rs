
mod custom_extractors;
pub mod middlewares;

use std::process::Output;
use tokio::process::Command;
pub use custom_extractors::path_extractor::PathExtractor;
pub use custom_extractors::json_extractor::JsonExtractor;
use crate::domain::ctx::Ctx;
use crate::errors::{ApiError, ApiResult, BaseError};

pub async fn sudo_exec(ctx:&Ctx, cmd: Vec<&str>) -> ApiResult<Output> {
    Command::new("sudo")
        .args(cmd.clone())
        .output().await.map_err(|err| ApiError {
        req_id: ctx.req_id(),
        error: BaseError::Execution { source: "here".to_string() },
    })
}