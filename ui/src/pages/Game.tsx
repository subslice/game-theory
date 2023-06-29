/**
 * The game page.
 * 
 * It puts together the game play components (i.e. GameRound) and the game info components
 * (i.e. Game Details and EventStream).
 * 
 * The players can take the following actions on this page:
 * 1. Enter a code for a game they want to join
 * 2. Configure and start a new game
 * 3. Play a round of a game (commit and reveal)
 * 4. View the game's details
 * 5. View the game's event stream
 * 6. Claim and withdraw game prize (if any)
 * 7. ...
 */

import { Flex, Box, Heading } from '@chakra-ui/react'

function Game() {
  return (
    <Box>
      Game

      {/* [GameRound] + [Game Info / EventStream] */}
    </Box>
  )
}

export default Game
