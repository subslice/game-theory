/**
 * Homepage of the Dapp.
 */

import { Flex, Box, Heading } from '@chakra-ui/react'
import MainMenu from '../components/MainMenu'
import EventStream from '../components/EventStream'

function Dashboard() {
  return (
    <Box>
      <Flex>
        <Box flex={1}>
          <MainMenu />
        </Box>
        <Box maxWidth={'450px'} flex={1}>
          <EventStream />
        </Box>
      </Flex>
    </Box>
  )
}

export default Dashboard
