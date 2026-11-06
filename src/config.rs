// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{Result, SentinelError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub service_name: String,
    pub mode: String,
    pub readiness_on_verified_only: bool,
    pub volatile_audit_capacity: usize,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            service_name: "incinerator-sentinel".to_string(),
            mode: "strict".to_string(),
            readiness_on_verified_only: true,
            volatile_audit_capacity: 1024,
        }
    }
}

impl ServiceConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path).map_err(|source| SentinelError::Io {
            path: path.to_path_buf(),
            source,
        })?;

        toml::from_str(&raw).map_err(|err| {
            SentinelError::InvalidConfig(format!("could not parse {}: {err}", path.display()))
        })
    }
}
