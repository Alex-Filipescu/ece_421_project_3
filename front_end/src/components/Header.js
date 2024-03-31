import { useState } from 'react';
import { NavLink } from 'react-router-dom';

export default function Header() {
  const [activeLink, setActiveLink] = useState(''); // Initial active link text

  const handleLinkClick = (linkText) => {
    setActiveLink(prevActiveLink => prevActiveLink === linkText ? '' : linkText);
  };

  return (
    <ul className="nav nav-underline justify-content-center">
      <li className="nav-item">
        <NavLink
          to="/home"
          className={`nav-link ${activeLink === 'Home' ? 'active' : ''}`}
          onClick={() => handleLinkClick('Home')}
        >
          Home!
        </NavLink>
      </li>
      <li className="nav-item">
        <NavLink
          to="/connect4"
          className={`nav-link ${activeLink === 'Connect4' ? 'active' : ''}`}
          onClick={() => handleLinkClick('Connect4')}
        >
          Connect4!
        </NavLink>
      </li>
      <li className="nav-item">
        <NavLink
          to="/tootOtto"
          className={`nav-link ${activeLink === 'TootOtto' ? 'active' : ''}`}
          onClick={() => handleLinkClick('TootOtto')}
        >
          Toot Otto!
        </NavLink>
      </li>
    </ul>
  );
}
