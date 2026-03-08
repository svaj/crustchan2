import React from 'react';
import './Header.css';

interface HeaderProps {
  title?: string;
  onMenuToggle?: () => void;
}

export const Header: React.FC<HeaderProps> = ({ title = 'Crustchan', onMenuToggle }) => {
  return (
    <header className="header">
      <div className="header-container">
        <div className="header-left">
          {onMenuToggle && (
            <button 
              className="menu-toggle" 
              onClick={onMenuToggle}
              aria-label="Toggle navigation menu"
            >
              ☰
            </button>
          )}
          <h1 className="header-title">{title}</h1>
        </div>
        <nav className="header-nav">
          <a href="/" className="nav-link">Home</a>
          <a href="/boards" className="nav-link">Boards</a>
          <a href="/settings" className="nav-link">Settings</a>
        </nav>
      </div>
    </header>
  );
};
