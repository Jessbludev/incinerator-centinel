// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

use serde::{Deserialize, Serialize};

use crate::{config::ServiceConfig, policy::PolicyBundle};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationReport {
    pub ok: bool,
    pub binary_hash: String,
    pub policy_hash: String,
    pub environment_fingerprint: String,
}

#[derive(Debug, Default)]
pub struct Attestor;

impl Attestor {
    pub fn new() -> Self {
        Self
    }

    pub fn measure(&self, bundle: &PolicyBundle, config: &ServiceConfig) -> AttestationReport {
        let binary_hash = blake3::hash(env!("CARGO_PKG_NAME").as_bytes())
            .to_hex()
            .to_string();
        let environment_fingerprint = blake3::hash(
            format!(
                "{}:{}:{}",
                env!("CARGO_PKG_VERSION"),
                config.service_name,
                config.mode
            )
            .as_bytes(),
        )
        .to_hex()
        .to_string();

        AttestationReport {
            ok: bundle.default_effect == "deny",
            binary_hash,
            policy_hash: bundle.bundle_hash.clone(),
            environment_fingerprint,
        }
    }
}
