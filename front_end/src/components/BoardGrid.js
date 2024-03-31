import React, { useState } from "react";
import { Grid, Paper, Box } from "@mui/material";
import axios from "axios"; // Import axios library

function BoardGrid({ cols, rows }) {
  const totalCells = cols * rows;
  const [cellValues, setCellValues] = useState(Array(totalCells).fill(null));
  const [message, setMessage] = useState(""); 

  const handleUserClick = async(index, newCellValues)=>{
    const columnNumber = index % cols;
  
    // Find the index of the first empty cell from the bottom of the column
    let emptyCellIndex = null;
    for (let i = rows - 1; i >= 0; i--) {
      const cellIndex = i * cols + columnNumber;
      if (!newCellValues[cellIndex]) {
        emptyCellIndex = cellIndex;
        break;
      }
    }
  
    if (emptyCellIndex !== null) {
      newCellValues[emptyCellIndex] = "X"; // Set the value of the empty cell to 'X'
      // Update the state with the modified cell values after the user's click
      setCellValues(newCellValues);

      try {
        // Send a POST request to the backend API endpoint with the column number
        const response = await axios.post("http://localhost:8000/api/getCol", {
          text: columnNumber.toString(),
        });
        console.log(response.data);
        setMessage(response.data);  
      } catch (error) {
        console.error("Error handling user click:", error);
      }
    } else {
      console.log("Column is full"); // Handle case when column is full
    }
    return newCellValues;

  };
  
  const handleBotMove = async (newCellValues) => {
    try {
      // Receive bot's move from the backend
      const response = await axios.get("http://localhost:8000/api/botMove");
      const { bot_move , message } = response.data;
      const botMoveColumn = Number(bot_move);
     
      // Find the index of the first empty cell from the bottom of the column specified by the bot
      let emptyCellIndex = null;
      for (let i = rows - 1; i >= 0; i--) {
        const cellIndex = i * cols + botMoveColumn;
        if (!newCellValues[cellIndex]) {
          emptyCellIndex = cellIndex;
          break;
        }
      }
  
      if (emptyCellIndex !== null) {
        newCellValues[emptyCellIndex] = "O"; // Set the value of the empty cell to 'O' for bot's move
        // Update the state with the modified cell values after the bot's move
        setCellValues(newCellValues);
        setMessage(message);

      } else {
        console.log("Bot's move column is full"); // Handle case when bot's move column is full
      }
    } catch (error) {
      console.error("Error handling bot move:", error);
    }
    return newCellValues;
  }

  const handleCellClick = async (index) => {
    const newCellValues = [...cellValues];
    let cellVals = await handleUserClick(index, newCellValues);
    console.log(cellVals);
    let botCellVals = await handleBotMove(cellVals);
    const newBotCellVals = [...botCellVals];
    setCellValues(newBotCellVals);
  };
  

  return (
    <Box display="flex" justifyContent="center">
      <Grid container spacing={0.5} style={{ maxWidth: "25%" }}>
        {[...Array(totalCells)].map((_, index) => (
          <Grid
            key={index}
            item
            xs={12 / cols}
            style={{ paddingBottom: "0", height: "auto" }}
          >
            <Paper
              onClick={() => handleCellClick(index)} // Add onClick handler to the Paper component
              style={{
                paddingTop: "60%", //for square
                textAlign: "center",
                display: "flex",
                justifyContent: "center",
                alignItems: "center",
                border: "1px solid black",
              }}
            >
              {cellValues[index]} {/* Display value of the cell */}
            </Paper>
          </Grid>
        ))}
      </Grid>
      <h2>Result {message}</h2>
    </Box>
  );
}

export default BoardGrid;
