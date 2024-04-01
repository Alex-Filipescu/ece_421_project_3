import { useState } from 'react';
import { NavLink } from 'react-router-dom';
import { makeStyles } from '@mui/styles';
import { AppBar, Toolbar, Tabs, Tab } from '@mui/material';

const useStyles = makeStyles((theme) => ({
  navLink: {
    textDecoration: 'none',
    color: 'white',
    '&.active': {
      fontWeight: 'bold',
    },
  },
  appBar: {
    backgroundColor: '#219EBC', // Light blue background color
  },
  toolbar: {
    justifyContent: 'center', // Center align the tabs
  },
}));

export default function Header() {
  const classes = useStyles();
  const [activeLink, setActiveLink] = useState('');

  const handleLinkClick = (linkText) => {
    setActiveLink((prevActiveLink) => (prevActiveLink === linkText ? '' : linkText));
  };

  return (
    <AppBar position="static" className={classes.appBar}>
      <Toolbar className={classes.toolbar}>
        <Tabs value={activeLink} onChange={(event, newValue) => setActiveLink(newValue)}>
          <Tab
            label="Home"
            value="Home"
            component={NavLink}
            to="/home"
            activeClassName={classes.active}
            className={classes.navLink}
            onClick={() => handleLinkClick('Home')}
          />
          <Tab
            label="Connect4"
            value="Connect4"
            component={NavLink}
            to="/connect4"
            activeClassName={classes.active}
            className={classes.navLink}
            onClick={() => handleLinkClick('Connect4')}
          />
          <Tab
            label="TootOtto"
            value="TootOtto"
            component={NavLink}
            to="/tootOtto"
            activeClassName={classes.active}
            className={classes.navLink}
            onClick={() => handleLinkClick('TootOtto')}
          />
        </Tabs>
      </Toolbar>
    </AppBar>
  );
}
