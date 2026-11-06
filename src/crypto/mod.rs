// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

use std::{fs, path::Path};

use crate::{Result, SentinelError};

#[derive(Debug, Default)]
pub struct ArtifactSigner;

impl ArtifactSigner {
    pub fn new() -> Self {
        Self
    }

    pub fn sign_placeholder(&self, path: &Path, key: &Path) -> Result<String> {
        let artifact = fs::read(path).map_err(|source| SentinelError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        let key_material = fs::read(key).map_err(|source| SentinelError::Io {
            path: key.to_path_buf(),
            source,
        })?;

        let mut material = Vec::with_capacity(artifact.len() + key_material.len());
        material.extend_from_slice(&artifact);
        material.extend_from_slice(&key_material);

        Ok(blake3::hash(&material).to_hex().to_string())
    }
}
