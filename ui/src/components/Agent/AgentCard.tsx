import React from 'react';
import { Agent } from '../../types';
import { Button } from '../Common';

interface AgentCardProps {
  agent: Agent;
  onEdit: () => void;
  onDelete: () => void;
}

export default function AgentCard({ agent, onEdit, onDelete }: AgentCardProps) {
  const statusColors = {
    active: 'bg-green-100 text-green-800',
    inactive: 'bg-gray-100 text-gray-800',
    error: 'bg-red-100 text-red-800',
  };

  return (
    <div className="bg-white p-4 rounded-lg shadow-md border">
      <div className="flex justify-between items-start">
        <div>
          <h3 className="font-semibold text-lg">{agent.name}</h3>
          <span className={`inline-block px-2 py-1 text-xs rounded-full ${statusColors[agent.status]}`}>
            {agent.status}
          </span>
        </div>
      </div>
      <p className="text-sm text-gray-500 mt-1">{agent.model}</p>
      <p className="text-gray-600 text-sm mt-2">{agent.description || 'No description'}</p>
      <div className="flex justify-between items-center mt-4">
        <span className="text-xs text-gray-500">{new Date(agent.created_at).toLocaleDateString()}</span>
        <div className="space-x-2">
          <Button size="sm" variant="secondary" onClick={onEdit}>
            Edit
          </Button>
          <Button size="sm" variant="danger" onClick={onDelete}>
            Delete
          </Button>
        </div>
      </div>
    </div>
  );
}