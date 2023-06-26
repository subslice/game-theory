import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';
import { options } from '@astar-network/astar-api';
import { Abi, CodePromise } from '@polkadot/api-contract'
import ABI from '../artifacts/public_good/public_good.json'
import fs from 'fs'

async function main() {
    const provider = new WsProvider('ws://localhost:9944');
    // OR
    // const provider = new WsProvider('wss://shiden.api.onfinality.io/public-ws');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    // Use the api
    // For example:
    console.log((await api.rpc.system.properties()).toHuman());

	const abi = new Abi(ABI, api.registry.getChainProperties())
	const metadata = JSON.parse(fs.readFileSync('./artifacts/public_good/public_good.contract', 'utf-8'))
	// Initialise the contract class
	const code = new CodePromise(api, abi, metadata.source.wasm)

	// maximum gas to be consumed for the instantiation. if limit is too small the instantiation will fail.
	const gasLimit = 100000n * 1000000n
	// a limit to how much Balance to be used to pay for the storage created by the instantiation
	// if null is passed, unlimited balance can be used
	const storageDepositLimit = null
	// used to derive contract address, 
	// use null to prevent duplicate contracts
	const salt = new Uint8Array()
	// balance to transfer to the contract account, formerly know as "endowment". 
	// use only with payable constructors, will fail otherwise. 
	const value = api.registry.createType('Balance', 1000)
	const initValue = 1;

	const tx = code.tx.new({ gasLimit, storageDepositLimit }, initValue)

	let address;

	const unsub = await tx.signAndSend(alicePair, ({ contract, status }) => {
	if (status.isInBlock || status.isFinalized) {
		address = contract.address.toString();
		unsub();
	}
	});

    process.exit(0);
}

main()