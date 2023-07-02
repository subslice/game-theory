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
npx @astar-network/swanky-cli contract compile router
npx @astar-network/swanky-cli contract compile public_good
npx @astar-network/swanky-cli contract compile dictator
npx @astar-network/swanky-cli contract compile rock_paper_scissors
