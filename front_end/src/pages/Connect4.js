import Header from '../components/Header'
import { IconButton } from "@mui/material";
import React, { useEffect, useState } from 'react';
import Connect4BoardGrid from '../components/Connect4BoardGrid';
import axios from 'axios';
import RefreshIcon from '@mui/icons-material/Refresh';
import MenuItem from '@mui/material/MenuItem';
import Button from '@mui/material/Button';
import Menu from '@mui/material/Menu';

function Connect4Page() {
  const [anchorEl, setAnchorEl] = React.useState(null);
  const open = Boolean(anchorEl);
  const [difficulty, setDifficulty] = React.useState('select'); // State to store the selected difficulty
  const [boardKey, setBoardKey] = useState(0); // Key for re-rendering the BoardGrid
  const [gridDisabled, setGridDisabled] = useState(true); // State to track if the grid is disabled

  useEffect(() => {
    // Define an async function to send the count to the backend
    const sendGameToBackend = async () => {
      try {
        // Send a get request to the backend API endpoint
        await axios.post("http://localhost:8000/api/getGame", { text: "Connect4" });
        await handleRefresh();
      } catch (error) {
        console.error("Error sending game name to the backend:", error);
      }
    };

    // Call the function to send the count to the backend
    sendGameToBackend();
  }, []);

  const handleRefresh = async () => {
    // Reload the page or perform any other actions to refresh the game
    setBoardKey(boardKey + 1);
    try {
      await axios.post("http://localhost:8000/api/refreshGame")
    } catch (error) {
      console.error("Error refreshing board:", error);
    }
  };

  const handleMenuClick = (event) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = async (difficultyLevel) => {
    if (difficultyLevel === "easy" || difficultyLevel === "medium" || difficultyLevel === "hard") {
      setDifficulty(difficultyLevel); // Set the selected difficulty
      setAnchorEl(null);
      //Send the selected difficulty to the backend
      try {
        await axios.post("http://localhost:8000/api/setDifficulty", { text: difficultyLevel });
        // Refresh the page
        handleRefresh();
        // Enable the grid
        setGridDisabled(false);
      } catch (error) {
        console.error("Error setting difficulty level:", error);
      }
    } else {
      setAnchorEl(null);
      // Disable the grid if the difficulty level is invalid
      setGridDisabled(true);
    }
  };

  return (
    <div>
      <Header />
      <div style={{ textAlign: 'center' }}>
        <h1>Connect 4 Game</h1>
        <Button
          id="basic-button"
          aria-controls={open ? 'basic-menu' : undefined}
          aria-haspopup="true"
          aria-expanded={open ? 'true' : undefined}
          onClick={handleMenuClick}
        >
          Bot Difficulty: {difficulty.toString() || 'Select'}
        </Button>

        <Menu
          id="basic-menu"
          anchorEl={anchorEl}
          open={open}
          onClose={handleClose}
          MenuListProps={{
            'aria-labelledby': 'basic-button',
          }}
        >
          <MenuItem onClick={() => handleClose('easy')} style={{ minWidth: '200px' }}>Easy</MenuItem>
          <MenuItem onClick={() => handleClose('medium')} style={{ minWidth: '200px' }}>Medium</MenuItem>
          <MenuItem onClick={() => handleClose('hard')} style={{ minWidth: '200px' }}>Hard</MenuItem>

        </Menu>
        <Connect4BoardGrid key={boardKey} cols={7} rows={6} disabled={gridDisabled} />
        <IconButton onClick={handleRefresh}>
          <RefreshIcon />
        </IconButton>
      </div>
    </div>
  );
}

export default Connect4Page;
