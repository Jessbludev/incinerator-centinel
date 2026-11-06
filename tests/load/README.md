# Pruebas de carga

Objetivo: demostrar que el plano de control del centinela no se convierte en
cuello de botella bajo trafico critico.

## SLI/SLO propuestos

| SLI | Objetivo |
|---|---|
| Latencia p99 de validacion de politica | < 5 ms |
| Latencia p99 de evaluacion de regla | < 1 ms |
| Disponibilidad del plano de control | 99.99% |
| Caida bajo carga sostenida | 0 reinicios no planificados |

## Escenarios

1. validacion de politica en bucle a tasa constante;
2. carga concurrente de multiples instancias de `selftest`;
3. rafaga (burst) con saturacion de CPU al 80%;
4. degradacion de red del plano de datos sin afectar al plano de control.

## Herramientas sugeridas

- `k6` para escenarios externos;
- `criterion` para microbenchmarks en Rust;
- `perf` y `pprof` para perfilado.

## Estado

Pendiente de implementacion: los escenarios requieren el servidor de
`serve` con un plano de datos real. Esta carpeta reserva el espacio y el plan.
