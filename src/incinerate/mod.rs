// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

#[derive(Debug, Default)]
pub struct Incinerator;

impl Incinerator {
    pub fn new() -> Self {
        Self
    }

    pub fn trigger(&self, reason: &str) {
        eprintln!("state=incinerated reason={reason}");
    }
}
