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

import { useState, ReactNode, CSSProperties } from 'react'
import { Flex, Box, Heading, Button } from '@chakra-ui/react'
import { useSearchParams } from 'react-router-dom'
import { ReactTerminal, TerminalContextProvider } from 'react-terminal'
import { Game as GameName, WhichGame, getFactoryByName, getContractByName, getAbiByName } from '../utils/ink.utils'
import { useApi, useDeployer, useWallet } from 'useink'
import { Abi } from '@polkadot/api-contract'

export enum Mode {
  Join = 'join',
  New = 'new',
  Spectate = 'spectate',
}

const tabbed: CSSProperties = { marginLeft: '100px' }
const spaced: CSSProperties = { width: '80px', textAlign: 'left', display: 'inline-block' }

function Game() {
  // parse the current query params of the page which renders this
  // react component
  const [queryParams] = useSearchParams()
  const queryObject = Object.fromEntries([...queryParams])
  // keep track of the game's state
  const [gameState, setGameState] = useState({} as Record<string, unknown>)
  // use the connected wallet to sign & send transactions
  const { isConnected, account } = useWallet()
  const deployer = useDeployer()
  const ApiUtil = useApi()

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

  const WelcomeMsg = (
    <span>
      <span>Welcome to <code>`GameSlice`</code>.</span> <br />
      Type "help" for all available commands. <br /><br /><br />

      <Button>Test</Button>
      <br /><br />
    </span>
  );

  const HelpMsg = (
    <span>
      <span><code style={spaced}>`whois`</code>: about this project.</span>
      <br /><br />
      <span><code style={spaced}>`help`</code>: you just typed this -_-</span>
      <br /><br />
      <span><code style={spaced}>`new`</code>: starts a new game of the type specified.</span>
      <br />
      <span style={tabbed}> For example <code>'new public_good'</code> can be used to start a new "public_good" game instance.</span>
      <br /><br />
      <span><code style={spaced}>`join`</code>: enables joining an existing game by its ID <small>(contract ID)</small>.</span>
      <br />
      <span style={tabbed}>For example <code>'join public_good bYbM...Z39S'</code> <small>(please use the full address)</small>.</span><br />
    </span>
  );

  const commands = {
    whois: "Game Theory + SubSlice = GameSlice",
    // cd: (directory: string) => `changed path to ${directory}`,
    help: HelpMsg,
    new: async (game: WhichGame) => {
      const availableGames = Object.values(WhichGame)
      
      if (!game) {
        return `Please specify a game to start`
      } else if (!availableGames.includes(game)) {
        return `Invalid game. Available games are: ${availableGames.map(g => `'${g}'`).join(', ')}`
      }

      // TODO: implement starting a new game

      const initFactory = getFactoryByName(GameName.PublicGood)
      const gameFactory = new initFactory(ApiUtil!.api, account.signer)

      // const result = await gameFactory.new({
      //   maxPlayers: 2,
      //   minPlayers: 2,
      //   minRoundContribution: 1_000,
      //   maxRoundContribution: 100_000,
      //   roundRewardMultiplier: 12,
      //   postRoundActions: false,
      //   roundTimeout: 10,
      //   maxRounds: 3,
      //   joinFee: 10_000,
      //   isRoundsBased: false,
      // })

      const abi = new Abi(getAbiByName(GameName.PublicGood), ApiUtil?.api.registry.getChainProperties())
      const result = await deployer.dryRun(abi, 'new', {
        maxPlayers: 2,
        minPlayers: 2,
        minRoundContribution: 1_000,
        maxRoundContribution: 100_000,
        roundRewardMultiplier: 12,
        postRoundActions: false,
        roundTimeout: 10,
        maxRounds: 3,
        joinFee: 10_000,
        isRoundsBased: false,
      })

      debugger

      return `Starting a ${game} game...`
    },
    join: (rawArgs: string) => {
      // TODO: implement joining a game by its ID
      const [game, contractId] = rawArgs.split(' ')
      const availableGames = Object.values(WhichGame)
      
      if (!availableGames.includes(game as WhichGame)) {
        return `Invalid game. Available games are: ${availableGames.map(g => `'${g}'`).join(', ')}`
      } else if (!contractId) {
        return `Invalid contract ID.`
      }

      return `Joining game ID = ${contractId} (${game})...`
    },
  };

  // {/* [GameRound] + [Game Info / EventStream] */}
  return (
    <Box padding={'50px'}>
      {/* Game */}
      {/* { _invokeGameByMode(queryObject.mode as Mode || Mode.New) } */}

      <Box height={'calc(100vh - 150px)'}>
        <TerminalContextProvider>
          <ReactTerminal
            commands={commands}
            theme={'light'}
            welcomeMessage={WelcomeMsg}
          />
        </TerminalContextProvider>
      </Box>
    </Box>
  )
}

export default Game
