import { expect, use } from "chai";
import chaiAsPromised from "chai-as-promised";
import PublicGoodFactory from "../typedContracts/public_good/constructors/public_good";
import PublicGood from "../typedContracts/public_good/contracts/public_good";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { GameConfigs } from '../typedContracts/public_good/types-arguments/public_good';

use(chaiAsPromised);

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944");
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" });

describe("flipper test", () => {
  const DEFAULT_MAX_PLAYERS = 2;
  const DEFAULT_MIN_PLAYERS = 2;
  const ACTORS = ['//Bob', '//Alice', '//Charlie'];

  let factory: PublicGoodFactory;
  let api: ApiPromise;
  let deployer: KeyringPair;
  
  let contract: PublicGood;

  // helpers
  const getSigner = (path: string) => keyring.addFromUri(path);
  const getSignerFromSecret = (secret: string) => keyring.addFromMnemonic(secret);

  beforeEach(async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider });
    deployer = keyring.addFromUri("//Alice");

    factory = new PublicGoodFactory(api, deployer);

    const configs: GameConfigs = {
      maxPlayers: DEFAULT_MAX_PLAYERS,
      minPlayers: DEFAULT_MIN_PLAYERS,
      minRoundContribution: 1_000,
      maxRoundContribution: 100_000,
      roundRewardMultiplier: 12,
      postRoundActions: false,
      roundTimeout: 10,
      maxRounds: 3,
      joinFee: 10_000,
      isRoundsBased: false,
    }

    contract = new PublicGood(
      (await factory.new(configs)).address,
      deployer,
      api
    );
  });

  after(async function tearDown() {
    await api.disconnect();
  });

  it("Should start with a 'Ready' state", async () => {
    let status = await contract.query.getStatus();
    expect(status.value.ok).to.equal('Ready');
  });

  it("Players can join", async () => {
    const signer = keyring.addFromUri(ACTORS[0]);
    await contract.withSigner(signer).tx.join(signer.address, {
      value: 10_000,
    });

    // the player that just joined should be represented in the contract state
    expect((await contract.query.getPlayers()).value.ok).to.include(signer.address);
  });

  it("Should not allow more than `max_players` to join", async () => {
    for (let i = DEFAULT_MAX_PLAYERS; i > 0; i--) {
      const signer = getSignerFromSecret(ACTORS[i - 1]);
      await contract.withSigner(signer).tx.join(signer.address, {
        value: 10_000,
      });
    }

    console.log('Players', (await contract.query.getPlayers()).value)

    // the player that just joined should be represented in the contract state
    let signer = getSignerFromSecret('//Eve');
    expect(
      contract.withSigner(signer).tx.join(signer.address, {
        value: 10_000,
      })
    ).to.throw;

    // TODO: check actual error
  });

  it("Should be able to start the game", async () => {
    // Add 2 players (minimum) to start the game 
    const BOB = getSigner(ACTORS[0]);
    await contract.withSigner(BOB).tx.join(BOB.address, {
      value: 10_000,
    });

    const ALICE = getSigner(ACTORS[1]);
    await contract.withSigner(ALICE).tx.join(ALICE.address, {
      value: 10_000,
    });

    // Start the game successfully
    await contract.withSigner(BOB).tx.startGame();
  });
  
  it("Should not start the game if `min_players` are not reached", async () => {
    const BOB = getSigner(ACTORS[0]);
    await contract.withSigner(BOB).tx.join(BOB.address, {
      value: 10_000,
    });

    // Start the game
    expect(contract.withSigner(BOB).tx.startGame()).to.throw;

    // TODO: check actual error
  });
});
