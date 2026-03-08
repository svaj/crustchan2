import React from 'react';
import './Footer.css';

export const Footer: React.FC = () => {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="footer">
      <div className="footer-container">
        <div className="footer-section">
          <h3>About</h3>
          <p>Crustchan is a modern imageboard for 2026.</p>
        </div>
        <div className="footer-section">
          <h3>Links</h3>
          <ul>
            <li><a href="/terms">Terms of Service</a></li>
            <li><a href="/privacy">Privacy Policy</a></li>
            <li><a href="/contact">Contact</a></li>
          </ul>
        </div>
        <div className="footer-section">
          <h3>Community</h3>
          <ul>
            <li><a href="/rules">Board Rules</a></li>
            <li><a href="/report">Report Content</a></li>
            <li><a href="/faq">FAQ</a></li>
          </ul>
        </div>
      </div>
      <div className="footer-bottom">
        <p>&copy; {currentYear} Crustchan. All rights reserved.</p>
      </div>
    </footer>
  );
};
