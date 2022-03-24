#!/bin/bash

docker run \
    -v cargo-cache:/root/.cargo/registry \
    -v "$PWD:/volume" \
    -e SQLX_OFFLINE=true \
    --rm -it clux/muslrust:nightly sh -c "cargo build --release && chown -R $(id -u):$(id -g) ./target"
