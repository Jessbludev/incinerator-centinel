# POLICY

## Reglas base

- `default_effect` debe ser `deny`;
- toda accion sensible debe tener TTL;
- los comodines deben evitarse;
- toda politica compilada debe ser verificable;
- una degradacion debe estar declarada y acotada.

## Flujo

1. escribir politica fuente en TOML;
2. validar;
3. compilar a bundle;
4. firmar;
5. verificar antes de servir.
