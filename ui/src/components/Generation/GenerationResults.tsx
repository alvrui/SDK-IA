import React from 'react';
import { GenerationResult } from '../../types';

interface GenerationResultsProps {
  generations: GenerationResult[];
}

export default function GenerationResults({ generations }: GenerationResultsProps) {
  if (generations.length === 0) return null;

  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold">Generation Results</h3>
      <div className="space-y-3">
        {generations.map(generation => (
          <div key={generation.id} className="border p-4 rounded-md bg-gray-50">
            <div className="flex justify-between items-start">
              <div>
                <span className={`inline-block px-2 py-1 text-xs rounded-full ${generation.status === 'completed' ? 'bg-green-100 text-green-800' : generation.status === 'failed' ? 'bg-red-100 text-red-800' : 'bg-yellow-100 text-yellow-800'}`}>
                  {generation.status}
                </span>
                <span className="ml-2 text-sm text-gray-500">{generation.element_type}</span>
              </div>
              <span className="text-xs text-gray-500">{new Date(generation.created_at).toLocaleString()}</span>
            </div>
            <p className="mt-2 whitespace-pre-wrap">{generation.content}</p>
          </div>
        ))}
      </div>
    </div>
  );
}