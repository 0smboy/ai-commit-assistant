// src/components/ui/alert.tsx
import React from "react";

interface AlertProps {
  children: React.ReactNode;
  className?: string;
}

export const Alert: React.FC<AlertProps> = ({ children, className = "" }) => {
  return (
    <div className={`rounded-lg border p-4 ${className}`}>{children}</div>
  );
};

export const AlertDescription: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <div className="text-sm">{children}</div>;
};

export const AlertTitle: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <h5 className="font-medium mb-1">{children}</h5>;
};
