import Header from '../components/Header'

import React, {useEffect} from 'react';
import BoardGrid from '../components/BoardGrid';
import axios from 'axios';

function Connect4Page() {
  useEffect(() => {
    // Define an async function to send the count to the backend
    const sendCountToBackend = async () => {
      try {
        // Send a get request to the backend API endpoint
        let gameName = "Connect4";
        await axios.post("http://localhost:8000/api/getGame", {text: "Connect4"});
      } catch (error) {
        console.error("Error sending game name to the backend:", error);
      }
    };

    // Call the function to send the count to the backend
    sendCountToBackend();
  }, []);
  return (
    <div>
      <Header/>
      <h1>Connect 4 Game</h1>
      <BoardGrid cols={7} rows={6} />
    </div>
  );
}

export default Connect4Page;