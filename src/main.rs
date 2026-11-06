// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

fn main() {
    if let Err(error) = incinerator_sentinel::cli::run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
