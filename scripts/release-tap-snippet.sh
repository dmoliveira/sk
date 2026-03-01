#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

usage() {
  printf 'Usage: %s <tag> [owner/repo]\n' "$(basename "$0")"
  printf 'Example: %s v0.2.0 dmoliveira/sk\n' "$(basename "$0")"
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

TAG="${1:-}"
REPO="${2:-}"

if [[ -z "$TAG" ]]; then
  printf 'Error: missing tag\n' >&2
  usage >&2
  exit 1
fi

if [[ -z "$REPO" ]]; then
  origin_url="$(git config --get remote.origin.url || true)"
  if [[ "$origin_url" =~ github.com[:/]([^/]+/[^/.]+)(\.git)?$ ]]; then
    REPO="${BASH_REMATCH[1]}"
  else
    printf 'Error: unable to infer owner/repo from origin remote\n' >&2
    printf 'Pass owner/repo as the second argument.\n' >&2
    exit 1
  fi
fi

url="https://github.com/${REPO}/archive/refs/tags/${TAG}.tar.gz"
tmp_file="/tmp/${REPO##*/}-${TAG}.tar.gz"

curl -fsSL "$url" -o "$tmp_file"
sha256="$(shasum -a 256 "$tmp_file" | awk '{print $1}')"

printf 'Release artifact\n'
printf '  repo:   %s\n' "$REPO"
printf '  tag:    %s\n' "$TAG"
printf '  url:    %s\n' "$url"
printf '  sha256: %s\n\n' "$sha256"

printf 'Formula snippet\n'
printf '  url "%s"\n' "$url"
printf '  sha256 "%s"\n' "$sha256"
