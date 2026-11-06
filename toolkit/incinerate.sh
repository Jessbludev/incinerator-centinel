#!/usr/bin/env sh
# Incineracion controlada: destruye estado sensible y detiene el servicio.
# Uso previsto en respuesta a compromiso (ver docs/RUNBOOK.md).
set -eu

PREFIX="${PREFIX:-/usr/local}"
BINARY="$PREFIX/bin/incinerator-sentinel"

echo "incinerate: iniciando contencion"

if command -v systemctl >/dev/null 2>&1; then
  systemctl stop incinerator-sentinel 2>/dev/null || true
fi

# Borrado de estado efimero.
rm -rf /var/lib/incinerator-sentinel 2>/dev/null || true

if [ -x "$BINARY" ]; then
  "$BINARY" selftest --strict >/dev/null 2>&1 || true
fi

echo "incinerate: contencion completada; reanudar solo tras verificacion externa"
