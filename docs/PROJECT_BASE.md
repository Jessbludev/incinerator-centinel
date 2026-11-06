# Incinerator Sentinel - Documentacion Base del Proyecto

Version: 1.0  
Estado del repositorio analizado: especificacion funcional y tecnica, sin implementacion materializada en codigo fuente  
Licencia objetivo: Apache-2.0

---

## 1. Resumen ejecutivo

Incinerator Sentinel es un guardian de servicios criticos orientado a entornos hostiles, con un modelo operativo de zero trust radical, aislamiento fuerte, politica deny-by-default, secretos efimeros, auditoria volatil y respuesta fail-closed.

La base disponible hoy no es un repositorio de aplicacion ejecutable. El workspace contiene una especificacion tecnica extensa y activos visuales, pero no contiene todavia `src/`, `Cargo.toml`, pipelines reales, tests ni scripts implementados. Por tanto, este documento actua como:

- documento maestro de producto y arquitectura;
- referencia de implementacion para levantar el proyecto base real;
- guia de operacion para entornos criticos y de alto trafico;
- definicion de requisitos no funcionales para que el sistema no solo "funcione", sino que opere con autonomia, resiliencia y contencion verificable.

El criterio de exito del proyecto no es disponibilidad ingenua a cualquier coste. El criterio correcto es:

1. verificar antes de confiar;
2. servir solo si la integridad es demostrable;
3. degradar de forma segura;
4. bloquear, aislar e incinerar estado sensible ante compromiso;
5. sostener carga elevada sin perder control, latencia operativa ni gobernanza.

---

## 2. Estado actual

### 2.1 Lo que existe

- especificacion tecnica principal de `Incinerator Sentinel`;
- documento de identidad visual de la mascota oficial `Guardaceniza`;
- asset SVG asociado.

### 2.2 Lo que no existe aun en el workspace

- crate Rust materializado;
- `Cargo.toml` y `Cargo.lock`;
- arbol `src/` con modulos reales;
- `toolkit/` funcional;
- `containers/` operativos;
- `nix/` implementado;
- CI/CD real;
- tests y harnesses de carga;
- runbooks ejecutables.

### 2.3 Implicacion practica

La especificacion es fuerte y coherente, pero el siguiente paso obligatorio es convertirla en un repositorio vivo. Este documento deja la base lista para ese salto y prioriza los elementos que haran al sistema autonomo y robusto bajo trafico critico.

---

## 3. Vision del sistema

Incinerator Sentinel debe ser un centinela de ejecucion para cargas y servicios criticos con estas propiedades:

- no confia implicitamente ni en operadores, ni en workers, ni en su propio proceso;
- no persiste secretos ni telemetria por defecto;
- no expone una superficie operativa innecesaria;
- no "se recupera" a ciegas despues de una inconsistencia de integridad;
- no sacrifica control ni seguridad para ganar disponibilidad aparente.

Su valor diferencial no es "ser otro supervisor", sino convertirse en una capa de contencion verificable entre el servicio critico y el entorno donde corre.

---

## 4. Objetivos de producto

### 4.1 Objetivos funcionales

- supervisar servicios criticos;
- aplicar politica estricta firmada;
- medir integridad de binario, bundle y entorno;
- gobernar acciones sensibles mediante quorums y TTL;
- gestionar secretos efimeros con zeroization;
- ejecutar aislamiento Linux duro;
- ofrecer export controlado de evidencia volatil;
- empaquetar releases reproducibles y firmadas.

### 4.2 Objetivos no funcionales

- operar sin telemetria remota por defecto;
- sostener escenarios de alto trafico sin colapso del plano de control;
- responder de forma automatica a fallos verificables;
- resistir errores de configuracion y errores operativos humanos;
- limitar blast radius ante compromiso;
- permitir auditoria minima sin exfiltrar datos sensibles.

### 4.3 No objetivos

- convertirse en plataforma de observabilidad generalista;
- permitir plugins dinamicos no firmados;
- introducir "modo permisivo" como comportamiento normal;
- depender de shell o descargas en runtime;
- usar logging persistente como mecanismo primario de operacion.

---

## 5. Principios arquitectonicos

- Zero trust radical: toda confianza debe revalidarse.
- Deny-by-default: toda accion no permitida se rechaza.
- Fail-closed: ante duda, detener o aislar.
- Minima superficie: menos binarios, menos privilegios, menos I/O.
- Reproducibilidad: release reconstruible y verificable.
- Secretos efimeros: no retencion, no persistencia, zeroize.
- Evidencia minima: hashes, estados, pruebas y export redacted.
- Gobernanza fuerte: firmas, quorum, TTL, expiracion automatica.
- Degradacion explicita: cualquier perdida de capacidades debe declararse.

---

## 6. Arquitectura base

### 6.1 Componentes nucleares

- `is-supervisor`: orquestacion principal y control de vida.
- `is-policy-engine`: evaluacion pura de politica, sin I/O.
- `is-attestor`: medicion y desafio criptografico.
- `is-secret-broker`: derivacion y custodia efimera de secretos.
- `is-enforcer`: seccomp, Landlock, namespaces, cgroups y limites.
- `is-volatile-audit`: anillo en RAM cifrado.
- `is-incinerator`: destruccion de secretos y aislamiento final.
- `is-config-compiler`: compilacion de politica a bundle firmado.
- `is-cli`: administracion, verificacion y empaquetado.

### 6.2 Separacion de responsabilidades

- el supervisor no evalua politica compleja;
- el policy engine no toca secretos ni filesystem;
- el enforcer no decide negocio;
- el attestor no ejecuta cargas utiles;
- el secret broker no se expone como API generalista;
- la auditoria no guarda payloads ni datos de usuario.

### 6.3 Plano de control y plano de datos

Para soportar trafico critico, el proyecto debe distinguir dos capas:

- plano de control: validacion, atestacion, politica, rotacion de claves, quorums, respuesta a incidentes;
- plano de datos: procesos o servicios protegidos que manejan la carga real.

Incinerator Sentinel no debe ponerse en medio del trafico de negocio como cuello de botella innecesario. Su funcion es custodiar, autorizar, verificar y contener. El trafico alto debe seguir fluyendo por el servicio protegido, mientras Sentinel impone:

- limites;
- readiness segura;
- aislamiento;
- revocacion;
- supervison;
- kill switch verificable.

### 6.4 Secuencia segura de arranque

1. cargar binario;
2. verificar firma;
3. medir binario, flags y entorno;
4. cargar y verificar policy bundle;
5. validar esquema y TTL;
6. inicializar secret broker;
7. derivar material efimero;
8. lanzar attestor;
9. desafiar a workers;
10. activar enforcer;
11. publicar readiness solo si todo lo anterior pasa.

Si cualquier paso falla:

- no entrar en servicio;
- incinerar estado sensible;
- devolver codigo de salida no exitoso;
- dejar solo evidencia volatil minima.

---

## 7. Autonomia operativa

La exigencia del proyecto es clara: no basta con que corra; debe ser autonomo. Eso implica automatizar decisiones, pero sin caer en auto-recuperacion ciega.

### 7.1 Que significa autonomia aqui

Autonomia no es "reiniciar siempre". Autonomia significa:

- detectar deriva de integridad;
- recalcular estado de confianza;
- revocar capacidad de servicio de forma automatica;
- rotar secretos efimeros sin intervencion humana;
- expirar privilegios temporales sin recordatorio externo;
- ejecutar self-test y auto-validacion antes de exponerse;
- reducir superficie de ataque durante degradacion;
- exportar evidencia minima cuando una politica lo autorice.

### 7.2 Autonomias permitidas

- reintentos internos acotados en inicializacion no sensible;
- rotacion automatica de claves efimeras;
- garbage collection de buffers y anillos volatiles;
- expiracion automatica de break-glass;
- pausa o drenaje de workers ante saturacion;
- rechazo de nuevas operaciones sensibles cuando el quorum o TTL ya no son validos.

### 7.3 Autonomias prohibidas

- volver a entrar en servicio tras fallo de atestacion sin verificacion externa;
- relajar politica por saturacion;
- abrir egreso de red como mecanismo de emergencia;
- habilitar logs persistentes por "diagnostico rapido";
- conservar secretos mas tiempo para simplificar recuperacion.

---

## 8. Diseño para alto trafico

El proyecto debe declararse apto para entornos criticos con alto trafico solo si evita que el plano de seguridad se convierta en cuello de botella.

### 8.1 Principio rector

El sistema debe proteger el servicio bajo carga, no competir con el servicio por CPU, memoria o latencia.

### 8.2 Reglas de diseño

- las decisiones de politica deben ser compiladas y evaluables en memoria;
- no debe haber I/O de red en la ruta critica del dataplane;
- la auditoria volatil debe usar buffer fijo y politica de sobrescritura;
- IPC debe ser local, autenticado y con framing sencillo;
- las verificaciones costosas deben hacerse en arranque, rotacion o eventos excepcionales, no por request de negocio;
- las acciones administrativas deben desacoplarse del path caliente.

### 8.3 Estrategia de escalado

- escalar horizontalmente el servicio protegido, no inflar indiscriminadamente el supervisor;
- mantener Sentinel como guardian local por nodo, pod o unidad protegida;
- separar workers por dominio de confianza;
- usar cgroups v2 para reservar CPU y memoria del plano de control;
- introducir limites de concurrencia para operaciones sensibles y administrativas.

### 8.4 Controles de backpressure

- colas de longitud fija para comandos internos;
- rechazo temprano cuando el sistema supera limites seguros;
- prioridad alta para incineracion, revocacion y señales de control;
- prioridad media para attestation y keepalives;
- prioridad baja para export y operaciones de diagnostico.

### 8.5 Recomendaciones concretas

- nunca bloquear el path de servicio por export de auditoria;
- nunca ejecutar compilacion de politica en runtime del servicio;
- nunca mezclar workers de negocio y modulos de verificacion en el mismo pool sin cuotas;
- nunca permitir que el buffer de auditoria compita con memoria de secretos.

---

## 9. Disponibilidad segura

Para entornos criticos, la disponibilidad debe definirse como "servicio integro y gobernado", no simplemente "proceso levantado".

### 9.1 Estados operativos

- `verified`: listo para servir, integridad valida;
- `degraded`: sirve con restricciones declaradas y aprobadas;
- `quarantined`: no admite nuevas operaciones sensibles;
- `incinerated`: estado sensible destruido, servicio no confiable;
- `maintenance-verified`: ventana temporal aprobada y acotada.

### 9.2 Politica de degradacion

Se puede degradar solo si:

- la degradacion esta prevista en politica;
- no compromete secreto ni integridad;
- no abre superficie nueva;
- queda marcada en auditoria volatil;
- tiene TTL y expiracion automatica.

### 9.3 Politica de failover

Recomendacion:

- usar failover externo del servicio protegido;
- no usar Sentinel como coordinador distribuido primario;
- si una instancia entra en `quarantined` o `incinerated`, el orquestador externo debe retirarla del balanceo;
- la reinstanciacion debe exigir boot seguro completo.

---

## 10. SLI, SLO y capacidad

Aunque la especificacion original no define objetivos cuantitativos cerrados, el proyecto base debe nacer con metas medibles.

### 10.1 SLI recomendados

- tiempo de arranque verificado;
- tiempo de attestation interna;
- latencia de evaluacion de politica;
- tiempo de revocacion e incineracion;
- porcentaje de arranques que alcanzan `verified`;
- uso maximo de memoria del anillo volatil;
- tiempo de retiro de readiness tras compromiso.

### 10.2 SLO iniciales recomendados

- readiness verificada en menos de 5 segundos en entorno nominal;
- retirada de readiness en menos de 1 segundo tras compromiso detectado;
- incineracion de secretos en menos de 500 ms tras trigger critico;
- p99 de evaluacion de politica por accion administrativa menor de 10 ms;
- cero escrituras persistentes no autorizadas en release;
- cero egreso de red por defecto en release.

### 10.3 Pruebas de capacidad obligatorias

- burst de operaciones administrativas concurrentes;
- rotacion de claves durante carga del servicio protegido;
- saturacion controlada de cola IPC;
- llenado total del anillo de auditoria;
- corrupcion del bundle bajo alta actividad;
- perdida y recuperacion de workers bajo backpressure;
- pruebas de CPU steal, memoria limitada y reloj anomalo.

---

## 11. Seguridad operacional

### 11.1 Secretos

- secretos maestros fuera del proceso o sellados;
- claves derivadas por sesion o por worker;
- material sensible en estructuras zeroized;
- prohibido almacenar claves en `String` o buffers sin borrado;
- prohibido swap para paginas sensibles.

### 11.2 Politica

- bundle compilado y firmado;
- validacion de esquema obligatoria;
- TTL maximo para acciones sensibles;
- wildcard prohibido salvo excepcion firmada y temporal;
- quorums para break-glass, export y acciones de alto impacto.

### 11.3 Aislamiento Linux

- systemd endurecido;
- namespaces y cgroups v2;
- seccomp;
- Landlock donde el kernel lo permita;
- capacidades vacias por defecto;
- filesystem read-only salvo tmpfs efimero;
- AF_UNIX como familia de direccion preferente;
- sin shell runtime.

---

## 12. Observabilidad sin traicionar el modelo

La ausencia de telemetria remota no elimina la necesidad de saber si el sistema esta sano. Obliga a una observabilidad austera y local.

### 12.1 Lo que si debe existir

- estados finitos de salud;
- readiness y liveness sobrias;
- contadores locales efimeros;
- hashes de artefactos cargados;
- eventos minimos en anillo cifrado;
- comandos autenticados de export redacted.

### 12.2 Lo que no debe existir

- exporters a Prometheus por defecto;
- trazas distribuidas;
- payload dumps;
- logs persistentes de debug;
- pipelines remotos de eventos.

### 12.3 Modelo recomendado

Exponer solo:

- `state`;
- `policy_hash`;
- `binary_hash`;
- `attestation_status`;
- `last_verified_at`;
- `quorum_window_status`;
- `degraded_reason_code`.

Nunca exponer:

- secretos;
- contenido de reglas sensibles;
- contenido de memoria;
- variables de entorno;
- detalles forenses no aprobados.

---

## 13. Modelo de despliegue

### 13.1 Modos de entrega

- binario Rust nativo endurecido;
- contenedor Alpine/musl;
- contenedor Ubuntu o distroless;
- build Nix/NixOS.

### 13.2 Modo operativo recomendado

El patron mas sano es "guardian local del workload":

- una instancia Sentinel por servicio critico o por grupo muy reducido de servicios;
- separado del balanceador y del dataplane publico;
- con responsabilidad sobre integridad, secretos, readiness y revocacion;
- con orquestacion externa encargada del failover.

### 13.3 Anti-patrones

- una sola instancia Sentinel para todo el cluster;
- usar Sentinel como proxy L7 generalista;
- centralizar secretos de todos los servicios en el mismo proceso;
- usar el mismo perfil de capacidades para todos los despliegues.

---

## 14. Roadmap de implementacion

### 14.1 Fase 0 - Bootstrap del repositorio

- crear `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` y `.cargo/config.toml`;
- materializar `src/main.rs`, `src/cli.rs`, `src/config.rs`;
- crear modulos `policy`, `attest`, `secret`, `enforce`, `audit`, `incinerate`, `crypto`;
- definir `README.md`, `SECURITY.md`, `LICENSE`, `NOTICE`;
- establecer `deny.toml`, CI minima y estructura `docs/`.

### 14.2 Fase 1 - MVP verificable

- parser y compilador de politica;
- verificacion de firma de bundle;
- flujo de arranque seguro;
- auditoria volatil basica;
- zeroization verificada;
- `serve`, `config validate`, `config compile`, `config verify`;
- self-test estricto;
- systemd unit endurecida.

### 14.3 Fase 2 - Aislamiento duro

- seccomp y namespaces;
- Landlock cuando aplique;
- cgroups v2 y cuotas;
- control de readiness y quarantine;
- toolkit de instalacion y verificacion.

### 14.4 Fase 3 - Alto trafico y autonomia

- colas internas con prioridades;
- backpressure y drenaje controlado;
- rotacion automatica de material efimero;
- tiempos maximos de attestation;
- integracion con orquestador para retiro inmediato del balanceo;
- chaos testing bajo carga.

### 14.5 Fase 4 - Supply chain fuerte

- build reproducible comprobada;
- SBOM;
- provenance;
- firmas cosign y Ed25519;
- pruebas cruzadas Alpine, Ubuntu y Nix;
- release bundle verificable.

---

## 15. Estructura de repositorio recomendada

```text
incinerator-sentinel/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── .cargo/config.toml
├── deny.toml
├── README.md
├── LICENSE
├── NOTICE
├── SECURITY.md
├── docs/
│   ├── SPEC.md
│   ├── POLICY.md
│   ├── THREAT_MODEL.md
│   ├── RUNBOOK.md
│   ├── SRE.md
│   └── CAPACITY.md
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── config.rs
│   ├── policy/
│   ├── attest/
│   ├── secret/
│   ├── enforce/
│   ├── audit/
│   ├── incinerate/
│   └── crypto/
├── tests/
│   ├── integration/
│   ├── load/
│   ├── chaos/
│   └── fuzz/
├── toolkit/
│   ├── install.sh
│   ├── verify-install.sh
│   ├── uninstall.sh
│   ├── incinerate.sh
│   └── systemd/
├── containers/
│   ├── Dockerfile.alpine
│   ├── Dockerfile.ubuntu
│   └── Dockerfile.distroless
├── nix/
│   ├── flake.nix
│   └── module.nix
└── examples/
    └── policy.strict.toml
```

---

## 16. Testing y validacion

### 16.1 Testing tecnico minimo

- unit tests de parser, TTL, quorum y firmas;
- property tests sobre invariantes de politica;
- fuzzing para parser, bundle e IPC;
- miri y clippy estrictos;
- integracion con systemd, Alpine, Ubuntu y Nix.

### 16.2 Testing para entornos criticos

- chaos tests con corrupcion del bundle;
- reinicio del supervisor bajo carga;
- intento de syscall prohibida;
- tmpfs lleno;
- reloj adelantado o atrasado;
- perdida de TPM o atestacion;
- split-brain entre workers;
- agotamiento de CPU y memoria reservada.

### 16.3 Testing de alto trafico

- carga sostenida del servicio protegido con Sentinel activo;
- burst de eventos administrativos;
- variacion brusca de concurrencia;
- drenaje y recovery del plano de control;
- medicion de latencia adicional introducida por Sentinel;
- verificacion de que no hay bloqueo del dataplane por auditoria ni attestation de rutina.

---

## 17. Runbooks minimos

### 17.1 Runbook de arranque

- verificar hashes y firmas;
- validar kernel y features disponibles;
- cargar bundle;
- ejecutar self-test;
- publicar readiness solo en `verified`.

### 17.2 Runbook de compromiso

- entrar en `quarantined`;
- congelar workers;
- zeroizar secretos;
- cerrar sockets;
- desmontar tmpfs sensible;
- emitir evidencia volatil firmada;
- salir con codigo de compromiso.

### 17.3 Runbook de mantenimiento

- solicitar break-glass mediante quorum;
- abrir ventana temporal con TTL;
- registrar evento volatil;
- revocar automaticamente al expirar;
- exigir boot seguro posterior si hubo cambios sensibles.

---

## 18. Riesgos honestos

- sin hardware seguro, la garantia frente a adversario fisico total es limitada;
- si el kernel esta comprometido, el aislamiento se reduce drasticamente;
- zero log complica forense historica, aunque protege confidencialidad;
- ZKP completo puede ser inviable en path caliente;
- supply chain depende parcialmente de crates y toolchains externos.

El proyecto debe documentar estos limites de forma frontal. La credibilidad tecnica se construye diciendo tambien donde termina la garantia.

---

## 19. Definition of Done del proyecto base

El proyecto base solo puede considerarse listo si cumple a la vez:

- repositorio Rust real materializado;
- arranque seguro verificable;
- politica compilada, firmada y validada;
- auditoria volatil operativa;
- zero telemetria y zero log persistente en release;
- secretos efimeros con zeroization probada;
- systemd endurecido;
- contenedores y Nix comprobados;
- pipeline CI/CD con fmt, clippy, tests, deny, audit y build reproducible;
- pruebas de caos y de carga con fail-closed demostrado;
- runbooks y documentacion de limites publicados.

---

## 20. Recomendacion final

La mejor forma de construir un proyecto "autonomo e imbatible" no es perseguir invulnerabilidad teorica, sino combinar:

- arquitectura simple;
- verificacion fuerte;
- blast radius pequeno;
- automatizacion segura;
- degradacion controlada;
- reintegro al servicio solo tras validacion;
- capacidad medida y testeada bajo carga real.

Incinerator Sentinel debe nacer como un sistema que protege incluso cuando decide no continuar. En entornos criticos, esa disciplina vale mas que cualquier promesa vacia de disponibilidad permanente.

---

## 21. Siguiente paso recomendado

Convertir esta documentacion en un repositorio bootstrap real con:

1. estructura de carpetas;
2. `Cargo.toml` inicial;
3. CLI minima;
4. esqueleto de modulos;
5. toolkit de instalacion;
6. CI base;
7. primer set de tests;
8. runbooks en `docs/`.
