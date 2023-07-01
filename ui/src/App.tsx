import './App.css'
import { Flex, Box, Heading } from '@chakra-ui/react'
import Navbar from './components/Navbar'
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import Game from './pages/Game'
import Dashboard from './pages/Dashboard'
import SciFiBackground from '../../images/sci-fi-bg-1.png'

function App() {
  return (
    <Box bgImage={SciFiBackground} bgRepeat={'none'} bgSize={'cover'}>
      <Navbar />
      <Router>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/game" element={<Game />} />
        </Routes>
      </Router>
    </Box>
  )
}

export default App
