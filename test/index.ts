// In this example we will deploy & interact with psp22 token to mint some tokens to the owner and get total supply.
import Contract from "./out/contracts/my_psp22";
import {ApiPromise, Keyring} from "@polkadot/api";
import Constructors from "./out/constructors/my_psp22";

async function main() {
	const api = await ApiPromise.create();

	const keyring = new Keyring({type: 'sr25519'});

	const aliceKeyringPair = keyring.addFromUri('//Alice');

	const constructors = new Constructors(api, aliceKeyringPair);

	const {address: TOKEN_ADDRESS} = await constructors.new('10000000000000000000');

	console.log('Contract deployed at:', TOKEN_ADDRESS);

	const contract = new Contract(TOKEN_ADDRESS, aliceKeyringPair, api);

	const totalSupply = await contract.query.totalSupply();
	const balance = await contract.query.balanceOf(aliceKeyringPair.address);

	console.log(`%c Total supply before minting: ${totalSupply.value.toString()}`, 'color: green');
	console.log(`%c Balance of Alice before minting: ${balance.value.toString()}`, 'color: green');

	const mintTx = await contract.tx.mint(aliceKeyringPair.address, '10000');

	const totalSupplyAfterMint = await contract.query.totalSupply();
	const balanceAfterMint = await contract.query.balanceOf(aliceKeyringPair.address);

	console.log(`%c Total supply after minting: ${totalSupplyAfterMint.value.toString()}`, 'color: green');
	console.log(`%c Balance of Alice after minting: ${balanceAfterMint.value.toString()}`, 'color: green');

	await api.disconnect();
}

main().then(() => {
	console.log('done');
});