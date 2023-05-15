#!/usr/bin/env bash

set -eu

cargo contract build --release --manifest-path contracts/router/Cargo.toml
cargo contract build --release --manifest-path contracts/public_good/Cargo.toml
cargo contract build --release --manifest-path contracts/rock_paper_scissors/Cargo.toml
cargo contract build --release --manifest-path contracts/dictator/Cargo.toml
