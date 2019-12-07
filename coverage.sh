#!/usr/bin/env bash

set -eux

PROJ_NAME=$(cat ./pygmaea/Cargo.toml | grep -E "^name" | sed -E 's/name[[:space:]]=[[:space:]]"(.*)"/\1/g' | sed -E 's/-/_/g')
rm -rf ./pygmaea/target/debug/deps/${PROJ_NAME}-*

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"

cargo +nightly build --verbose
cargo +nightly test --verbose

grcov ./pygmaea/target/debug/deps -s . -t lcov --llvm --branch --ignore-not-existing --ignore-dir "/*" -o lcov.info
genhtml -o report/ --show-details --highlight --ignore-errors source --legend lcov.info
