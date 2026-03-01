#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

index_file="docs/index.md"

required_links=(
	"./security-for-ai-agents.md"
	"./agent-security-faq.md"
	"./agent-security-checklist.md"
)

if [[ ! -f "$index_file" ]]; then
	printf 'Missing required file: %s\n' "$index_file" >&2
	exit 1
fi

missing=0
for link in "${required_links[@]}"; do
	if ! grep -Fq "$link" "$index_file"; then
		printf 'Missing required link in %s: %s\n' "$index_file" "$link" >&2
		missing=1
	fi
done

if [[ "$missing" -ne 0 ]]; then
	exit 1
fi

printf 'Doc link guard passed: %s\n' "$index_file"
