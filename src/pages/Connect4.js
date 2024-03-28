import Header from '../components/Header'

import React from 'react';
import Gridi from '../components/Gridi';

function MyPage() {
  return (
    <div>
      <Header/>
      <h1>Connect 4 Game</h1>
      <Gridi cols={7} rows={6} />
    </div>
  );
}

export default MyPage;