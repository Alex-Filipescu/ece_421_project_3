import Header from '../components/Header'
import { makeStyles } from '@mui/styles';

// const useStyles = makeStyles((theme) => ({
//     navLink: {
//       textDecoration: 'none',
//       color: 'white',
//       '&.active': {
//         fontWeight: 'bold',
//       },
//     },
//     appBar: {
//       backgroundColor: '#219EBC', // Light blue background color
//     },
//     toolbar: {
//       justifyContent: 'center', // Center align the tabs
//     },
//   }));

export default function Home(){
    
    return(
        <>
        <Header/>
        <div style={{ textAlign: 'center' }}>

        <h2>Welcome!</h2>
        <h3>Here are some instructions:</h3>  
        <h3>Have fun!</h3>  
        </div>
        </>
    )
}