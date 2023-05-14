# Game Theory with ink!

> Submission for the Encode x Polkadot Hackathon 2023

This repo contains an implementation of [games within the Game Theory field](https://en.wikipedia.org/wiki/List_of_games_in_game_theory).

Only a couple of games are currently implemented, deployable and playable from the list linked above, however, the implementation
is done such that it follows a standardized set of traits (see `./logics/traits` directory) defined within the reusable (across contracts) code
such as type and trait definitions. More details in the [Project Components section](#project-components).

Taking such an approach means more games can be easily added to extend this project beyond the current hackathon submission.

---

## Motivation

Open-source software such as [oTree](https://www.otree.org/), a behavioural-research platform, is often used to create such games
in order to simulate, with students or participants, stakes seen in real-life economics. 

Data from such simulations is then utilized in research papers or case-studies.

This motivates us to create a decentralized version of such games, potentially growing into a network for research purposes
in later stages of this project. The benefit of using blockchain for such a purpose is the following:

* **Transparency and Trust** of data quality and authenticity
* **High Availability**
* (potentially) Incentive-driven participation of players

Sources for economics research can vary and be equally valid. Could blockchain and (very) light yet economically motivated games
introduce a fun way to contribute to this?

---

## Overview

### Overview & Architecture

![](./images/architecture.png)

### Project Components

1. **Traits**: the `traits` crate defines the behaviours which every game must provide as well as common types.
    * The `trait-defintion` of `Lifecycle` specifies the methods which must be implemented by each game.
    * Common types include `Error`, `GameStatus`, `RoundStatus`, `GameRound` and `GameConfigs`.

2. **Games**: the games themselves are example contract implementations (`rock_paper_scissors` and `public_good`) of the `Lifecycle` trait.
    * Each __instance__ of a game contract represents a single game play.
    * To play the game again, a new __instance__ must be launched.
    * The contract should self-destruct once the game is completed and final round's winnings are issued.

3. [post hackathon] **Router**: the `games_router` create defines a generic contract which the UI will invoke when launching 
new games as well as when having high-level interactions with the games' contracts.
   * It does not contain game specific code.
   * It is extensible to allow future games to be added.

4. [post hackathon] **Typescript/React UI**
   a. Add **Subsquid** for event indexing and lookup
   b. Add Admin / Analytics dashboard

---

## Getting Started

> Note: please use the `nightly-2023-02-07` cargo toolchain channel. See [Cargo.toml](./Cargo.toml)

### Testing

Simply run `cargo test` to run tests of all the crates / packages.

To run a specific contract's tests, use the `-p` (or `--package`) flag, for example:

```shell
cargo test -p public_good
```

or 

```shell
cargo test -p rock_paper_scissors
```

or 

alternatively `cd` into that contract's directory and run `cargo test` within the package code.

### Building Contracts

Since this is a workspace, each contract currently needs to be built independently into WASM / ABI.

Use the following command to a build a contract:

```shell
cargo contract build --release --manifest-path contracts/SOME_CONTRACT_FOLDER/Cargo.toml
```

### Deploying Contracts

[//]: # (TODO: write)


---

## Next Steps

- [ ] ...
- [ ] ...
- [ ] ...
- [ ] ...

