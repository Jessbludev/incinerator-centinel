// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, SentinelError>;

#[derive(Debug, Error)]
pub enum SentinelError {
    #[error("I/O error on {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("invalid policy: {0}")]
    InvalidPolicy(String),
    #[error("integrity violation: {0}")]
    IntegrityViolation(String),
    #[error("unsupported operation: {0}")]
    Unsupported(String),
}
