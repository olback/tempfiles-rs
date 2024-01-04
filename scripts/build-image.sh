#!/bin/sh

set -e

cd frontend
npm i
npm run build
cd ..

# Target AMD Zen 2 arch
RUSTFLAGS="-Ctarget-cpu=x86-64-v3" cargo build --release --target x86_64-unknown-linux-musl
cp ~/.cargo-target/x86_64-unknown-linux-musl/release/tempfiles-rs .
docker build -t registry.olback.dev/olback/tempfiles-rs .
docker push registry.olback.dev/olback/tempfiles-rs
