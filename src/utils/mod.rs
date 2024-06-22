mod custom_extractors;
pub mod middlewares;

use crate::errors::{Result, BaseError};
pub use custom_extractors::json_extractor::JsonExtractor;
pub use custom_extractors::path_extractor::PathExtractor;
// pub use custom_extractors::cookie_extractor::ExtractJwt;
use std::process::Output;
use tokio::process::Command;

pub async fn sudo_exec(cmd: Vec<&str>) -> Result<Output> {
    Command::new("sudo")
        .args(cmd.clone())
        .output()
        .await
        .map_err(|_err| BaseError::Execution { source: _err.to_string() })
}
