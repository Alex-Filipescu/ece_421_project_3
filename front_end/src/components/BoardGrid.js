import React, { useState } from "react";
import { Grid, Paper, Box } from "@mui/material";
import axios from "axios"; // Import axios library

function BoardGrid({ cols, rows }) {
  const totalCells = cols * rows;
  const [cellValues, setCellValues] = useState(Array(totalCells).fill(null));
  const [message, setMessage] = useState(""); 
  const [hoveredColumn, setHoveredColumn] = useState(null);
  const [gameOver, setGameOver] = useState(false); // State to track game over

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
        if (response.data == "0" || response.data == "1" || response.data == "2"){
          setGameOver(true);
        }
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
        if (message == "0" || message == "1" || message == "2"){
          setGameOver(true);
        }
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
    if (gameOver) return;

    const newCellValues = [...cellValues];

    //check if column is full
    const columnNumber = index % cols;
    let isColumnFull = true;

    // Check each row in the column
    for (let i = rows - 1; i >= 0; i--) {
      const cellIndex = i * cols + columnNumber;
      if (!newCellValues[cellIndex]) {
        // If any cell in the column is empty, the column is not full
        isColumnFull = false;
        break;
      }
    }

    if (isColumnFull) {
      setMessage("Column is full");
      return; // Return without modifying the cell values
    }

    let cellVals = await handleUserClick(index, newCellValues);
    setTimeout(async () => {
      let botCellVals = await handleBotMove(cellVals);
      const newBotCellVals = [...botCellVals];
      setCellValues(newBotCellVals);
    }, 800);
  };

  const handleColumnHover = (columnNumber) => {
    setHoveredColumn(columnNumber);
  };

  const handleColumnHoverLeave = () => {
    setHoveredColumn(null);
  };

  const isCellHovered = (index) => {
    const columnNumber = index % cols;
    return columnNumber === hoveredColumn;
  };

  return (
    <div>
    <Box display="flex" justifyContent="center">
      <Grid container spacing={0.5} style={{ maxWidth: "50%" }}>
        {[...Array(totalCells)].map((_, index) => (
          <Grid
            key={index}
            item
            xs={12 / cols}
            style={{ paddingBottom: "0", height: "auto" }}
          >
            <Paper
                id={`cell-${index}`}
                onClick={() => handleCellClick(index)}
                onMouseEnter={() => handleColumnHover(index % cols)}
                onMouseLeave={handleColumnHoverLeave}
                style={{
                  padding: "25%",
                  textAlign: "center",
                  fontSize: "24px", // Set font size to 24px
                  display: "flex",
                  justifyContent: "center",
                  alignItems: "center",
                  border: "1px solid black",
                  transition: "background-color 0.3s",
                  backgroundColor: isCellHovered(index) ? "#D2EEFC " : "white",
                }}
              >
              {cellValues[index] || "\u00A0"} {/* Display value of the cell */}
            </Paper>
          </Grid>
        ))}
      </Grid>
    </Box>
    <h3>Result: {message}</h3>
    </div>
  );
}

export default BoardGrid;
