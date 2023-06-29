/**
 * The dashboard's main menu.
 * 
 * It shows the main actions a user can take once they've connected a wallet.
 * 1. Create a new game
 * 2. Join an existing game
 * 3. View your profile
 * 4. List existing games
 */

import { Flex, Box, Heading } from '@chakra-ui/react'
import { useWallet } from 'useink'

function MainMenu() {
    const { isConnected } = useWallet()

    if (!isConnected) {
        return (
            <Box>
                Please connect a wallet to get started
            </Box>
        )
    }

    return (
        <Box>
            MainMenu
        </Box>
    )
}

export default MainMenu