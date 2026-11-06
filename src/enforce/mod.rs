// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

use crate::{config::ServiceConfig, Result, SentinelError};

#[derive(Debug, Default)]
pub struct Enforcer;

impl Enforcer {
    pub fn new() -> Self {
        Self
    }

    pub fn prepare_runtime(&self, config: &ServiceConfig) -> Result<()> {
        if config.mode != "strict" {
            return Err(SentinelError::InvalidConfig(
                "bootstrap base only supports strict mode".to_string(),
            ));
        }

        Ok(())
    }
}
