# Game Theory Games with !ink

> Submission for the Encode x Polkadot Hackathon 2023

---

## Overview

### Packages

The project contains a few components that together compose the overall experience.

1. Traits: the `traits` crate defines the behaviours which every game must provide and common types.
    * The `trait-defintion` of `GameLifecycle` specifies the methods which must be implemented by each game.
    * Common types include `Error`, `GameStatus`, `RoundStatus`, `GameRound` and `GameConfigs`.

2. Router: the `games_router` create defines a generic contract which the UI will invoke when launching new games as well as when having high-level interactions with the games' contracts.
    * It does not contain game specific code.
    * It is extensible to allow future games to be added.

3. Games: the games themselves are example contract implementations (`game_rock_paper_scissors` and `game_public_good`) of the `GameLifecycle` trait.
    * Each __instance__ of a game contract represents a single game play.
    * To play the game again, a new __instance__ must be launched.
    * The contract should self-destruct once the game is completed and final round's winnings are issued.


### General Notes

Because the Router is agnostic to the exact games, other games which follow the `GameLifecycle` trait can be implemented, launched on-chain and added to the Router to be used publicly.

There is also no absolute need for the router other than making the instantiation of games easier as well as some future generic behaviour which simplifies contract interaction.


### Architecture Diagram

![](./images/architecture.png)


---

## Getting Started

### Testing

Simply run `cargo test` to run tests of all the crates / packages.

To run a specific contract's tests, use the `-p` flag, for example:

```shell
cargo test -p game_public_good
```

or `cd` into that contract's directory and run `cargo test` within.


### Building Contracts

Since this is a workspace, each contract currently needs to be built independantly into WASM / ABI.

Use the following command to a build a contract:

```shell
cargo contract build --release --manifest-path contracts/SOME_CONTRACT_FOLDER/Cargo.toml
```
