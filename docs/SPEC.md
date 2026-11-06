# SPEC

Incinerator Sentinel es un centinela de servicios criticos orientado a zero trust, deny-by-default y fail-closed.

## Componentes

- supervisor;
- policy engine;
- attestor;
- secret broker;
- enforcer;
- volatile audit;
- incinerator;
- CLI.

## Estado del bootstrap

Esta base implementa:

- estructura del crate;
- CLI inicial;
- policy compiler bootstrap;
- selftest;
- contratos minimos de runtime.

Esta base todavia no implementa:

- seccomp real;
- Landlock real;
- TPM;
- signing Ed25519 final;
- formato de bundle final;
- auditoria cifrada completa.
