# Pruebas de caos

Objetivo: verificar que ante cualquier fallo el sistema **falla cerrado**
(deniega, aisla e incinera estado sensible) y nunca "soft fail".

## Escenarios

| Escenario | Resultado esperado |
|---|---|
| Corromper el policy bundle | rechazo + exit no cero |
| Politica sin `default_effect = "deny"` | rechazo + exit no cero |
| Bundle sin firma | no se considera confiable |
| Formato de entrada desconocido | rechazo + exit no cero |
| Binario modificado en caliente | medicion inconsistente + incineracion |
| Syscall prohibida | bloqueo + evento volatil |
| Reloj fuera de rango | rechazo de atestacion |
| Perdida de TPM | degradacion declarada o fail-closed |

## Invariante global

```text
Ningun escenario debe terminar en estado "serving" sin verificacion.
```

## Estado

Pendiente de implementacion: se ejecutaran como tests de integracion una vez
exista el enforcer real. Esta carpeta reserva el espacio y el plan.
