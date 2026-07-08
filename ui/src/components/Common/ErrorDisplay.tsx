import React from 'react';

interface ErrorDisplayProps {
  error: string | null;
  onRetry?: () => void;
}

export default function ErrorDisplay({ error, onRetry }: ErrorDisplayProps) {
  if (!error) return null;
  return (
    <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-md">
      <p className="font-medium">{error}</p>
      {onRetry && (
        <button onClick={onRetry} className="mt-2 text-sm bg-red-100 hover:bg-red-200 px-3 py-1 rounded">
          Retry
        </button>
      )}
    </div>
  );
}