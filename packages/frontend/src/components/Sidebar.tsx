import React from 'react';
import './Sidebar.css';

interface SidebarProps {
  isOpen: boolean;
  onClose: () => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ isOpen, onClose }) => {
  return (
    <>
      {isOpen && <div className="sidebar-overlay" onClick={onClose} />}
      <aside className={`sidebar ${isOpen ? 'open' : ''}`}>
        <nav className="sidebar-nav">
          <a href="/boards" className="sidebar-link">All Boards</a>
          <a href="/favorites" className="sidebar-link">Favorites</a>
          <a href="/history" className="sidebar-link">History</a>
          <hr className="sidebar-divider" />
          <a href="/settings" className="sidebar-link">Settings</a>
          <a href="/help" className="sidebar-link">Help</a>
        </nav>
      </aside>
    </>
  );
};
