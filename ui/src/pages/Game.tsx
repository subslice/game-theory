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

import { Flex, Box, Heading, useQuery } from '@chakra-ui/react'
import { useSearchParams } from 'react-router-dom'

export enum Mode {
  Join = 'join',
  New = 'new',
  Spectate = 'spectate',
}

function Game() {
  // parse the current query params of the page which renders this
  // react component
  const [queryParams] = useSearchParams()
  const queryObject = Object.fromEntries([...queryParams])

  const _renderNewGame = () => {
    return (
      <Box>
        <Heading as="h2" size="md">New Game</Heading>
      </Box>
    )
  }

  const _invokeGameByMode = (mode: Mode) => {
    switch (mode) {
      case Mode.New:
        return _renderNewGame()
      case Mode.Join:
      case Mode.Spectate:
      default:
        return <></>
    }
  }

  // {/* [GameRound] + [Game Info / EventStream] */}
  return (
    <Box>
      Game

      { _invokeGameByMode(queryObject.mode as Mode || Mode.New) }
    </Box>
  )
}

export default Game
