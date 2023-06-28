import React from "react"
import ReactDOM from "react-dom/client"
import App from "./App.tsx"
import "./index.css"
import { UseInkProvider } from "useink"
import { RococoContractsTestnet, ShibuyaTestnet } from "useink/chains"
import { Custom, Chain } from 'useink/chains'
import { ChakraProvider } from '@chakra-ui/react'

// Source: https://use.ink/frontend/configuration/
const LocalChain: Chain = {
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

export default App

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <UseInkProvider
      config={{
        dappName: "GameSlice",
        chains: [RococoContractsTestnet, ShibuyaTestnet, LocalChain],
        // caller: {
          // // An optional default caller address to be used before a user connects their wallet.
          // default: "5EyR7vEk7DtvEWeefGcXXMV6hKwB8Ex5uvjHufm466mbjJkR", 
        // },
      }}
    >
      <ChakraProvider>
        <App />
      </ChakraProvider>
    </UseInkProvider>
  </React.StrictMode>
)
