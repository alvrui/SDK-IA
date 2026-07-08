import React, { useState } from 'react';
import { useAgents } from '../../hooks';
import { Button, LoadingSpinner, ErrorDisplay, Modal } from '../Common';
import AgentForm from './AgentForm';
import AgentCard from './AgentCard';

export default function AgentList() {
  const { agents, loading, error, fetchAgents, createAgent, deleteAgent } = useAgents();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedAgent, setSelectedAgent] = useState(null);

  const handleCreate = async (data) => {
    await createAgent(data);
    setIsModalOpen(false);
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this agent?')) {
      await deleteAgent(id);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold">Agents</h1>
        <Button onClick={() => { setSelectedAgent(null); setIsModalOpen(true); }}>
          Create Agent
        </Button>
      </div>

      {loading && <LoadingSpinner text="Loading agents..." />}
      {error && <ErrorDisplay error={error} onRetry={fetchAgents} />}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {agents.map(agent => (
          <AgentCard
            key={agent.id}
            agent={agent}
            onEdit={() => { setSelectedAgent(agent); setIsModalOpen(true); }}
            onDelete={() => handleDelete(agent.id)}
          />
        ))}
      </div>

      <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title={selectedAgent ? 'Edit Agent' : 'Create Agent'}>
        <AgentForm agent={selectedAgent} onSubmit={handleCreate} onClose={() => setIsModalOpen(false)} />
      </Modal>
    </div>
  );
}