import './App.css'
import { Flex, Box, Heading } from '@chakra-ui/react'
import Navbar from './components/Navbar'
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import Game from './pages/Game'
import Dashboard from './pages/Dashboard'

function App() {
  return (
    <Box>
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
