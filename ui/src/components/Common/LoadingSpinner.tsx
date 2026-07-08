import React from 'react';

interface LoadingSpinnerProps {
  size?: 'sm' | 'md' | 'lg';
  text?: string;
}

export default function LoadingSpinner({ size = 'md', text }: LoadingSpinnerProps) {
  const sizes = { sm: 'w-4 h-4 border-2', md: 'w-8 h-8 border-4', lg: 'w-12 h-12 border-6' };
  return (
    <div className="flex items-center justify-center space-x-2">
      <div className={`animate-spin rounded-full border-blue-500 border-t-transparent ${sizes[size]}`} />
      {text && <span className="text-gray-600">{text}</span>}
    </div>
  );
}