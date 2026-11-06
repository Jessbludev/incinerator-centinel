# RUNBOOK

## Arranque

1. validar la politica;
2. compilar el bundle;
3. ejecutar `selftest --strict`;
4. arrancar `serve` con config y policy;
5. publicar readiness solo en estado `verified`.

## Compromiso

1. entrar en cuarentena;
2. zeroizar material efimero;
3. cortar sockets de control;
4. retirar la instancia del balanceo;
5. reiniciar solo tras nueva verificacion.

## Mantenimiento

1. aplicar cambio con TTL;
2. registrar evento volatil;
3. expirar permisos temporales;
4. volver a estado verificado.
