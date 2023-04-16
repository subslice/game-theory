#!/usr/bin/env bash

set -eu

cargo +stable contract build --manifest-path contracts/game_public_good/Cargo.toml
cargo +stable contract build --manifest-path contracts/game_rock_paper_scissors/Cargo.toml
cargo +stable contract build --manifest-path contracts/games_router/Cargo.toml
cargo +stable contract build