import Header from '../components/Header'
import BoardGrid from '../components/BoardGrid';

export default function TootOtto(){
    return(
        <div>
      <Header/>
      <h1>Toot Otto </h1>
      <BoardGrid cols={6} rows={4} />
    </div>
    )
}