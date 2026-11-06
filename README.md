# Incinerator Sentinel

Zero-trust guardian for critical services.

## Estado

Este repositorio ya contiene una base real del proyecto:

- crate Rust inicial;
- CLI operativa con comandos base;
- modulos esqueleto para politica, atestacion, secretos, enforcement, auditoria e incineracion;
- toolkit de instalacion;
- CI minima;
- contenedores de referencia;
- estructura documental para especificacion, threat model y runbooks.

La implementacion actual prioriza una base limpia, extensible y alineada con la documentacion maestra. Todavia no implementa hardening completo de kernel, atestacion TPM ni criptografia de release final.

## Principios

- zero trust radical;
- deny-by-default;
- fail-closed;
- cero telemetria remota por defecto;
- cero log persistente por defecto;
- secretos efimeros y zeroization;
- despliegue reproducible y verificable.

## Estructura

```text
src/
  cli.rs
  config.rs
  error.rs
  lib.rs
  main.rs
  policy/
  attest/
  secret/
  enforce/
  audit/
  incinerate/
  crypto/
```

## Comandos base

```bash
incinerator-sentinel serve --config ./config.toml --policy ./examples/policy.strict.toml
incinerator-sentinel selftest --strict
incinerator-sentinel config validate --input ./examples/policy.strict.toml
incinerator-sentinel config compile --input ./examples/policy.strict.toml --output ./policy.bundle
incinerator-sentinel config verify --bundle ./policy.bundle
incinerator-sentinel bundle create --binary ./incinerator-sentinel --policy ./policy.bundle --output ./release.tar.zst
incinerator-sentinel bundle verify --input ./release.tar.zst
```

## Inicio rapido

1. Instala Rust 1.85 o superior compatible con `rust-toolchain.toml`.
2. Ejecuta `cargo fmt`, `cargo clippy` y `cargo test`.
3. Valida una politica de ejemplo.
4. Extiende los modulos internos con enforcement y attestation reales.

## Documentacion

- `docs/SPEC.md`
- `docs/POLICY.md`
- `docs/THREAT_MODEL.md`
- `docs/RUNBOOK.md`
- `/workspace/incinerator-sentinel-doc-base.md`

## Mascota

La identidad visual oficial es `Guardaceniza`, ubicada en `assets/mascot/`.

## Licencia

Codigo: Apache-2.0.
