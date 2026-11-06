// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

#![no_main]

use libfuzzer_sys::fuzz_target;

use incinerator_sentinel::policy::PolicyCompiler;

fuzz_target!(|data: &[u8]| {
    if let Ok(source) = std::str::from_utf8(data) {
        // El validador debe rechazar entradas invalidas sin entrar en panico.
        let _ = PolicyCompiler::validate_source_str(source);
    }
});
