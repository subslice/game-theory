/**
 * Homepage of the Dapp.
 */

import { Flex, Box, Heading } from '@chakra-ui/react'
import MainMenu from '../components/MainMenu'
import EventStream from '../components/EventStream'

function Dashboard() {
  return (
    <Box minHeight={'calc(100vh - 75px)'}>
      <Flex>
        <Box margin={'20px'} flex={1}>
          <MainMenu />
        </Box>
        <Box margin={'20px'} maxWidth={'450px'} flex={1}>
          <EventStream />
        </Box>
      </Flex>
    </Box>
  )
}

export default Dashboard
