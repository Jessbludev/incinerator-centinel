#!/usr/bin/env sh
set -eu

PREFIX="/usr/local"
BINARY=""
POLICY=""
PUBKEY=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    --binary) BINARY="$2"; shift 2 ;;
    --policy) POLICY="$2"; shift 2 ;;
    --pubkey) PUBKEY="$2"; shift 2 ;;
    --prefix) PREFIX="$2"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 1 ;;
  esac
done

test -n "$BINARY"
test -n "$POLICY"
test -n "$PUBKEY"
test -f "$BINARY"
test -f "$POLICY"
test -f "$PUBKEY"

install -d "$PREFIX/bin"
install -d /etc/incinerator-sentinel
install -m 0755 "$BINARY" "$PREFIX/bin/incinerator-sentinel"
install -m 0644 "$POLICY" /etc/incinerator-sentinel/policy.bundle
install -m 0644 ./toolkit/systemd/incinerator-sentinel.service /etc/systemd/system/incinerator-sentinel.service

echo "installed bootstrap layout under $PREFIX and /etc/incinerator-sentinel"
