import React, { useState } from 'react';
import './MainLayout.css';
import { Header } from './Header';
import { Sidebar } from './Sidebar';
import { Footer } from './Footer';

interface MainLayoutProps {
  children: React.ReactNode;
  title?: string;
  showSidebar?: boolean;
}

export const MainLayout: React.FC<MainLayoutProps> = ({
  children,
  title = 'Crustchan',
  showSidebar = true,
}) => {
  const [sidebarOpen, setSidebarOpen] = useState(false);

  return (
    <div className="main-layout">
      <Header 
        title={title}
        onMenuToggle={() => setSidebarOpen(!sidebarOpen)}
      />
      <div className="layout-container">
        {showSidebar && (
          <Sidebar 
            isOpen={sidebarOpen}
            onClose={() => setSidebarOpen(false)}
          />
        )}
        <main className="main-content">
          {children}
        </main>
      </div>
      <Footer />
    </div>
  );
};
