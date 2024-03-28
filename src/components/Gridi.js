import React from 'react';
import { Grid,Paper,Box } from '@mui/material';

function Gridi({ cols, rows }) {
  const totalCells = cols * rows;
  return (
<Box display="flex" justifyContent="center">
      <Grid container spacing={0.5} style={{ maxWidth: '25%'}}>
        {[...Array(totalCells)].map((_, index) => (
          <Grid key={index} item xs={12 / cols} style={{ paddingBottom: '0', height: 'auto' }}>
            <Paper
              style={{ 
                paddingTop: '60%', //for square
                textAlign: 'center',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                border:'1px solid black'
              }}
            >
              {index + 1} {/* Display index or content */}
            </Paper>
          </Grid>
        ))}
      </Grid>
    </Box>
  );
}

export default Gridi;