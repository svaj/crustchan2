import React from 'react';
import './Container.css';

interface ContainerProps {
  children: React.ReactNode;
  maxWidth?: 'sm' | 'md' | 'lg' | 'xl' | 'full';
  className?: string;
}

export const Container: React.FC<ContainerProps> = ({
  children,
  maxWidth = 'lg',
  className = '',
}) => {
  return (
    <div className={`container container-${maxWidth} ${className}`}>
      {children}
    </div>
  );
};
