// import logo from './logo.svg';
import './App.css';
import React, { useEffect,useState } from 'react';
import {BrowserRouter, Routes, Route} from 'react-router-dom'
import Home from './pages/Home'
import Connect4 from './pages/Connect4'
import TootOtto from './pages/TootOtto'

function App() {

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
