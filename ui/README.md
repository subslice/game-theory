
## Game Slice UI

This is a typescript + React sub-project that handles building the UI (web for now) components necessary to
interact with the deployed Ink! smart contracts (on Astar/Shibuya/Swanky).

### Getting Started

Assuming you've already got a local network running (see [ZombieNet section in main README](../README.md)), here are the steps to start the UI server:

1. Copy the env file and edit the values

```
cp .env.example .env
```

> TODO: add overview of required env vars (e.g. network address)

2. Install dependencies

```
yarn
```

3. Run the UI dev server

```
yarn dev
```

> TODO: add more details to this section


### Generating TS Types from Smart Contracts

> TODO: write up with steps for type extraction from Ink!


### Connecting a Wallet (Web3 Login)

> TODO: write up about session generation


### Interacting with Ink! Smart Contracts

> TODO: ...


### TODO

- [ ] Create a login wrapper with hooks
- [ ] Maintain login state with app refresh (be mindful of security vs. accessibility)
- [ ] Create basic "Game" component which expects children components as props & is configurable
    - [ ] Maintains game lifecycle / flow
    - [ ] Organizes the components' layout
    - [ ] Invokes lifecycle methods
- [ ] Create "GameState" component, simply displays game info such as turns, events, etc...
- [ ] ...
