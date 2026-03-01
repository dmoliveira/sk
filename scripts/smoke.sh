#!/usr/bin/env bash
set -euo pipefail
IFS=$'
	'

if [[ -z "${SK_BIN:-}" ]]; then
  cargo build --quiet
  SK_BIN="./target/debug/sk"
fi

KEY="sk-smoke-$$"
VALUE="smoke-$(date +%s)"

"$SK_BIN" selfcheck
"$SK_BIN" add -k "$KEY" -v "$VALUE" --force
got=$("$SK_BIN" get -k "$KEY")
if [[ "$got" != "$VALUE" ]]; then
  printf 'Smoke test failed: value mismatch
' >&2
  exit 1
fi
"$SK_BIN" remove -k "$KEY" -y
printf 'Smoke test OK
'
