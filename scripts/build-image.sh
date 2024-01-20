#!/bin/sh

set -e

TARGET_DIR="${CARGO_TARGET_DIR:-"target"}"

cd frontend
npm i --legacy-peer-deps
npm run build
cd ..

# Target x86-64-v3
RUSTFLAGS="-Ctarget-cpu=x86-64-v3" cargo build --release --target x86_64-unknown-linux-gnu
cp $TARGET_DIR/x86_64-unknown-linux-gnu/release/tempfiles-rs .
docker build -t registry.olback.dev/olback/tempfiles-rs .
docker push registry.olback.dev/olback/tempfiles-rs
