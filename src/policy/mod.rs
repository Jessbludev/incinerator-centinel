// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{Result, SentinelError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyBundle {
    pub bundle_hash: String,
    pub default_effect: String,
    pub rule_count: usize,
    pub is_signed: bool,
    pub source_hint: String,
}

impl PolicyBundle {
    pub fn synthetic() -> Self {
        Self {
            bundle_hash: blake3::hash(b"synthetic-policy").to_hex().to_string(),
            default_effect: "deny".to_string(),
            rule_count: 1,
            is_signed: true,
            source_hint: "synthetic".to_string(),
        }
    }

    pub fn to_wire(&self) -> String {
        format!(
            "bundle_hash={}\ndefault_effect={}\nrule_count={}\nis_signed={}\nsource_hint={}\n",
            self.bundle_hash, self.default_effect, self.rule_count, self.is_signed, self.source_hint
        )
    }

    pub fn export_json(&self, redact: bool) -> String {
        let source_hint = if redact { "redacted" } else { &self.source_hint };
        format!(
            "{{\"bundle_hash\":\"{}\",\"default_effect\":\"{}\",\"rule_count\":{},\"is_signed\":{},\"source_hint\":\"{}\"}}",
            self.bundle_hash, self.default_effect, self.rule_count, self.is_signed, source_hint
        )
    }
}

pub struct PolicyCompiler;

impl PolicyCompiler {
    pub fn validate_source(input: &Path) -> Result<()> {
        let raw = fs::read_to_string(input).map_err(|source| SentinelError::Io {
            path: input.to_path_buf(),
            source,
        })?;

        if !raw.contains("default_effect") {
            return Err(SentinelError::InvalidPolicy(
                "missing required field default_effect".to_string(),
            ));
        }

        if !raw.contains("deny") {
            return Err(SentinelError::InvalidPolicy(
                "policy must remain deny-by-default".to_string(),
            ));
        }

        Ok(())
    }

    pub fn compile(input: &Path) -> Result<PolicyBundle> {
        Self::validate_source(input)?;
        let raw = fs::read_to_string(input).map_err(|source| SentinelError::Io {
            path: input.to_path_buf(),
            source,
        })?;
        let bundle_hash = blake3::hash(raw.as_bytes()).to_hex().to_string();
        let rule_count = raw.matches("[[rule]]").count();

        Ok(PolicyBundle {
            bundle_hash,
            default_effect: "deny".to_string(),
            rule_count,
            is_signed: true,
            source_hint: input.display().to_string(),
        })
    }

    pub fn load_bundle_or_source(input: &Path) -> Result<PolicyBundle> {
        let raw = fs::read_to_string(input).map_err(|source| SentinelError::Io {
            path: input.to_path_buf(),
            source,
        })?;

        if raw.contains("default_effect") {
            return Self::compile(input);
        }

        if raw.contains("bundle_hash=") {
            return Ok(parse_wire_bundle(&raw, input));
        }

        Err(SentinelError::InvalidPolicy(format!(
            "unsupported policy or bundle format: {}",
            input.display()
        )))
    }
}

fn parse_wire_bundle(raw: &str, input: &Path) -> PolicyBundle {
    let mut bundle = PolicyBundle {
        bundle_hash: blake3::hash(raw.as_bytes()).to_hex().to_string(),
        default_effect: "deny".to_string(),
        rule_count: 0,
        is_signed: false,
        source_hint: input.display().to_string(),
    };

    for line in raw.lines() {
        if let Some(value) = line.strip_prefix("bundle_hash=") {
            bundle.bundle_hash = value.to_string();
        } else if let Some(value) = line.strip_prefix("default_effect=") {
            bundle.default_effect = value.to_string();
        } else if let Some(value) = line.strip_prefix("rule_count=") {
            bundle.rule_count = value.parse().unwrap_or(0);
        } else if let Some(value) = line.strip_prefix("is_signed=") {
            bundle.is_signed = value == "true";
        } else if let Some(value) = line.strip_prefix("source_hint=") {
            bundle.source_hint = value.to_string();
        }
    }

    bundle
}
