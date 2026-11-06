#!/usr/bin/env sh
# Crea un release bundle firmado a partir del binario y el policy bundle.
set -eu

BINARY="${1:-target/x86_64-unknown-linux-musl/release/incinerator-sentinel}"
POLICY="${2:-policy.bundle}"
OUTPUT="${3:-dist/incinerator-sentinel.tar.zst}"

test -f "$BINARY" || { echo "falta el binario: $BINARY" >&2; exit 1; }
test -f "$POLICY" || { echo "falta el policy bundle: $POLICY" >&2; exit 1; }

mkdir -p "$(dirname "$OUTPUT")"

tar --zstd -cf "$OUTPUT" "$BINARY" "$POLICY"

if command -v sha256sum >/dev/null 2>&1; then
  sha256sum "$OUTPUT" > "$OUTPUT.sha256"
fi

echo "bundle: creado $OUTPUT"
