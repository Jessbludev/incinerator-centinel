// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

use zeroize::Zeroizing;

#[derive(Debug)]
pub struct SecretBroker {
    session_material: Zeroizing<Vec<u8>>,
}

impl Default for SecretBroker {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretBroker {
    pub fn new() -> Self {
        Self {
            session_material: Zeroizing::new(vec![0u8; 32]),
        }
    }

    pub fn rotate_session_material(&mut self) {
        let material = blake3::hash(b"rotate-session-material");
        self.session_material.copy_from_slice(material.as_bytes());
    }

    pub fn fingerprint(&self) -> String {
        blake3::hash(&self.session_material).to_hex().to_string()
    }
}
