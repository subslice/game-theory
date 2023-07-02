import { Custom, Chain } from 'useink/chains'
import PublicGood from '../../../typedContracts/public_good/contracts/public_good'
import PublicGoodFactory from '../../../typedContracts/public_good/constructors/public_good'

// Source: https://use.ink/frontend/configuration/
export const LocalChain: Chain = {
    ...Custom,
    // NOTE: The value of 'id' should be 'custom'. ChainId must be of a known value that we 
    // have already defined in useink/chains. We do this because chainId is used as an 
    // argument in many hooks and we want to prevent bugs due to mispelled chain names. 
    // For example: `useBlockHeader('astart')` would return undefined because `astart` 
    // is not a chainId. `astar` is the correct name. ChainId has known values so that 
    // TypeScript will show you your error before runtime.
    id: 'custom',
    name: 'Local Chain',
    rpcs: ['ws://localhost:9944'],
}

export enum Game {
    PublicGood = 'PublicGood',
}

export enum WhichGame {
    PublicGood = 'public_good',
}

export const getContractByName = (contract: Game) => {
    switch (contract) {
        case Game.PublicGood:
            return PublicGood
        default:
            throw new Error('Unknown contract')
    }
}

export const getFactoryByName = (factory: Game) => {
    switch (factory) {
        case Game.PublicGood:
            return PublicGoodFactory
        default:
            throw new Error('Unknown factory')
    }
}
