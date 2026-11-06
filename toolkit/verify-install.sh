#!/usr/bin/env sh
# Verificacion post-instalacion de Incinerator Sentinel.
set -eu

PREFIX="${PREFIX:-/usr/local}"
BINARY="$PREFIX/bin/incinerator-sentinel"

fail() {
  echo "verify: FALLO $1" >&2
  exit 1
}

test -x "$BINARY" || fail "binario no ejecutable en $BINARY"

echo "verify: binario presente"

if command -v systemctl >/dev/null 2>&1; then
  systemctl is-enabled incinerator-sentinel >/dev/null 2>&1 \
    || fail "unidad systemd no habilitada"
  echo "verify: unidad systemd habilitada"
fi

"$BINARY" selftest --strict || fail "selftest estricto fallo"

echo "verify: OK"
