import React from "react"
import ReactDOM from "react-dom/client"
import App from "./App.tsx"
import "./index.css"
import { UseInkProvider } from "useink"
import { Development, RococoContractsTestnet, ShibuyaTestnet } from "useink/chains"
import { ChakraProvider } from '@chakra-ui/react'
import { LocalChain } from './utils/ink.utils.ts'
import theme from './theme.ts'

export default App

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <UseInkProvider
      config={{
        dappName: "GameSlice",
        chains: [RococoContractsTestnet, ShibuyaTestnet, LocalChain, Development],
        // caller: {
          // // An optional default caller address to be used before a user connects their wallet.
          // default: "5EyR7vEk7DtvEWeefGcXXMV6hKwB8Ex5uvjHufm466mbjJkR", 
        // },
      }}
    >
      <ChakraProvider theme={theme}>
        <App />
      </ChakraProvider>
    </UseInkProvider>
  </React.StrictMode>
)
