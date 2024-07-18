#!/usr/bin/env bash
set -euo pipefail

inplace() {
  local file="$1"
  shift
  local tmp
  tmp=$(mktemp)
  echo "Running: $@"
  "$@" < "$file" > "$tmp"
  mv "$tmp" "$file"
  rm -f "$tmp"
}

main() {
  inplace pkg/package.json jq '.exports["."].import = "./rust_rectangle_dividing.js"'
  inplace pkg/package.json jq '.exports["."].require = "./rust_rectangle_dividing.js"'
  inplace pkg/package.json jq '.exports["."].types = "./rust_rectangle_dividing.d.ts"'
  inplace pkg/package.json jq '.type = "module"'
  inplace pkg/package.json jq '.module = "rust_rectangle_dividing.js"'
  inplace pkg/package.json jq '.types = "rust_rectangle_dividing.d.ts"'
  inplace pkg/package.json jq '.name = "@kitsuyui/rectangle-dividing"'
}

main
