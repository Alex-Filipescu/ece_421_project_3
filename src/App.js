import logo from './logo.svg';
import './App.css';
import React, { useState } from 'react';
import {BrowserRouter, Routes, Route} from 'react-router-dom'
import Home from './pages/Home'
import Connect4 from './pages/Connect4'
import TootOtto from './pages/TootOtto'
import init, { greet } from './pkg/rust_component';

function App() {
  // const [greeting, setGreeting] = useState('');

  // const handleGreet = () => {
  //   const message = greet('React-Rust Developer');
  //   setGreeting(message);
  // };

  return (
    <div>
      <BrowserRouter>
      <Routes>
        <Route index element ={<Home/>}/>
        <Route path = "/home" element = {<Home/>}/>
        <Route path = "/connect4" element = {<Connect4/>}/>
        <Route path = "/tootOtto" element = {<TootOtto/>}/>
      </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
