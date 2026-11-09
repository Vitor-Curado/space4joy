#!/usr/bin/env bash

set -Eeuo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT_DIR"

#echo "==> Building frontend"

(
    cd client
    npm ci
    npm run build
)

#echo "==> Generating application assets"

cargo run --bin assets

#echo "==> Compiling Rust application"

cargo build

#echo "==> Build completed successfully"

clear