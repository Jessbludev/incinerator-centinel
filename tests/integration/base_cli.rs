// SPDX-License-Identifier: Apache-2.0
// Incinerator Sentinel

use std::path::PathBuf;

use incinerator_sentinel::SentinelError;
use incinerator_sentinel::policy::PolicyCompiler;

fn example_policy() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/policy.strict.toml")
}

#[test]
fn example_policy_compiles_with_deny_default() {
    let bundle =
        PolicyCompiler::compile(&example_policy()).expect("la politica ejemplo debe compilar");

    assert_eq!(bundle.default_effect, "deny");
    assert_eq!(bundle.rule_count, 1);
    assert!(
        !bundle.is_signed,
        "un bundle compilado no debe auto-declararse firmado sin firma real"
    );
}

#[test]
fn policy_without_deny_default_is_rejected() {
    let dir = tempfile::tempdir().expect("tmpdir");
    let path = dir.path().join("bad.toml");
    std::fs::write(&path, "version = 1\ndefault_effect = \"allow\"\n").expect("write");

    let err = PolicyCompiler::validate_source(&path).expect_err("debe rechazarse");

    assert!(matches!(err, SentinelError::InvalidPolicy(_)));
}

#[test]
fn wire_bundle_is_not_trusted_as_signed() {
    let dir = tempfile::tempdir().expect("tmpdir");
    let path = dir.path().join("bundle.wire");
    std::fs::write(
        &path,
        "bundle_hash=abc\ndefault_effect=deny\nrule_count=1\nis_signed=false\nsource_hint=x\n",
    )
    .expect("write");

    let bundle = PolicyCompiler::load_bundle_or_source(&path).expect("carga de wire bundle");

    assert!(
        !bundle.is_signed,
        "un wire bundle sin firma no puede considerarse firmado"
    );
}

#[test]
fn unknown_format_is_rejected() {
    let dir = tempfile::tempdir().expect("tmpdir");
    let path = dir.path().join("garbage.txt");
    std::fs::write(&path, "no soy una politica\n").expect("write");

    let err = PolicyCompiler::load_bundle_or_source(&path).expect_err("debe rechazarse");

    assert!(matches!(err, SentinelError::InvalidPolicy(_)));
}
