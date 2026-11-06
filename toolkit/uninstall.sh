#!/usr/bin/env sh
# Desinstalacion de Incinerator Sentinel.
set -eu

PREFIX="${PREFIX:-/usr/local}"

if command -v systemctl >/dev/null 2>&1; then
  systemctl disable --now incinerator-sentinel 2>/dev/null || true
fi

rm -f /etc/systemd/system/incinerator-sentinel.service
rm -f "$PREFIX/bin/incinerator-sentinel"
rm -rf /etc/incinerator-sentinel
rm -rf /var/lib/incinerator-sentinel

if command -v systemctl >/dev/null 2>&1; then
  systemctl daemon-reload || true
fi

echo "uninstall: Incinerator Sentinel retirado de $PREFIX"
