// import logo from './logo.svg';
import './App.css';
import React, { useEffect,useState } from 'react';
import {HashRouter, Routes, Route} from 'react-router-dom'
import Home from './pages/Home'
import Connect4 from './pages/Connect4'
import TootOtto from './pages/TootOtto'

function App() {

  return (
    <div>
      <HashRouter>
      <Routes>
        <Route index element ={<Home/>}/>
        <Route path = "/home" element = {<Home/>}/>
        <Route path = "/connect4" element = {<Connect4/>}/>
        <Route path = "/tootOtto" element = {<TootOtto/>}/>
      </Routes>
      </HashRouter>
    </div>
  );
}

export default App;
