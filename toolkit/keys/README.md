# Claves de firma

Este directorio documenta la gestion de claves. No versionar material privado.

## Reglas

- Claves privadas nunca en el repositorio ni en disco de produccion.
- Uso preferente de HSM o almacen de claves externo.
- Operaciones sensibles requieren quorum N-de-M.
- Rotacion periodica documentada y con expiracion.

## Contenido esperado en despliegue

```text
signing.key   # privada, fuera del repo
signing.pub   # publica, distribuible
```

Ver `docs/RUNBOOK.md` para el procedimiento de rotacion.
