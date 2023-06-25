#!/usr/bin/env bash

set -eu

# install node deps
yarn
# run a check with Swanky CLI
npx @astar-network/swanky-cli check

# compile all contracts with Swanky cli
cargo contract build --release --manifest-path contracts/router/Cargo.toml
cargo contract build --release --manifest-path contracts/public_good/Cargo.toml
cargo contract build --release --manifest-path contracts/rock_paper_scissors/Cargo.toml
cargo contract build --release --manifest-path contracts/dictator/Cargo.toml

# generate types
npx @astar-network/swanky-cli contract typegen router
npx @astar-network/swanky-cli contract typegen public_good
npx @astar-network/swanky-cli contract typegen dictator
npx @astar-network/swanky-cli contract typegen rock_paper_scissors
