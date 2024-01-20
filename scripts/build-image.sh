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

date=$(date +"%Y-%m-%d")

docker build --pull --no-cache -t registry.olback.dev/olback/tempfiles-rs:$date .
docker tag registry.olback.dev/olback/tempfiles-rs:$date registry.olback.dev/olback/tempfiles-rs:latest
docker push registry.olback.dev/olback/tempfiles-rs:$date
docker push registry.olback.dev/olback/tempfiles-rs:latest
