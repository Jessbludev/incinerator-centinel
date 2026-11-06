# Fuzzing

Harness de `cargo-fuzz` para los parsers de entrada.

## Targets

- `policy_parser`: valida que `PolicyCompiler::validate_source_str` nunca entra
  en panico ante entradas arbitrarias.

## Requisitos

```bash
cargo install cargo-fuzz
```

## Ejecucion

Desde este directorio:

```bash
cargo +nightly fuzz run policy_parser
```

## Invariante

Ante cualquier entrada, la funcion debe devolver `Ok` o `Err`, nunca abortar.
