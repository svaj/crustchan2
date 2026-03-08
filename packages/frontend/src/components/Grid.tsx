import React from 'react';
import './Grid.css';

interface GridProps {
  children: React.ReactNode;
  columns?: number | { mobile?: number; tablet?: number; desktop?: number };
  gap?: 'sm' | 'md' | 'lg';
  className?: string;
}

export const Grid: React.FC<GridProps> = ({
  children,
  columns = 1,
  gap = 'md',
  className = '',
}) => {
  let gridClass = `grid grid-gap-${gap}`;
  
  if (typeof columns === 'number') {
    gridClass += ` grid-cols-${columns}`;
  } else {
    if (columns.mobile) gridClass += ` grid-cols-mobile-${columns.mobile}`;
    if (columns.tablet) gridClass += ` grid-cols-tablet-${columns.tablet}`;
    if (columns.desktop) gridClass += ` grid-cols-desktop-${columns.desktop}`;
  }

  return (
    <div className={`${gridClass} ${className}`}>
      {children}
    </div>
  );
};
