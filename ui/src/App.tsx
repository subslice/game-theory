import './App.css'
import { Flex, Box, Heading } from '@chakra-ui/react'
import Navbar from './components/Navbar'

function App() {
  return (
    <Box>
      <Navbar />
      <Flex flexDirection={'column'}>
        <Box>
          <Heading>
            Game Slice
          </Heading>
        </Box>
      </Flex>
    </Box>
  )
}

export default App
