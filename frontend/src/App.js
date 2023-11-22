import React from 'react'
import './App.css'
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import Home from './pages'
import Table from './pages/table'
import Order from './pages/order'

function App() {
  return (
    <Router>
      <Routes>
        <Route exact path='/'                    element={<Home />} />
        <Route path='/tables/:table_number'      element={<Table />} />
        <Route path='/tables/:table_number/:id'  element={<Order />} />
      </Routes>
    </Router>
  )
}

export default App
