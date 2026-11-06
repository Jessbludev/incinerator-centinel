// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolatileEvent {
    pub actor: String,
    pub action: String,
    pub result: String,
}

#[derive(Debug)]
pub struct VolatileAudit {
    capacity: usize,
    events: VecDeque<VolatileEvent>,
}

impl VolatileAudit {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            events: VecDeque::with_capacity(capacity),
        }
    }

    pub fn record(&mut self, actor: &str, action: &str, result: &str) {
        if self.events.len() == self.capacity {
            let _ = self.events.pop_front();
        }

        self.events.push_back(VolatileEvent {
            actor: actor.to_string(),
            action: action.to_string(),
            result: result.to_string(),
        });
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}
