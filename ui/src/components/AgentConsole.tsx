import React, { useState, useEffect } from 'react';
import { Agent, AgentStatus, AgentCreate, AgentUpdate, ChatMessage } from '../types/agent';
import { listAgents, createAgent, updateAgent, deleteAgent, sendAgentMessage, getAgentServiceStatus, getModelDisplayName, getStatusColor, getStatusText, getDefaultAgent } from '../api/agents';

export default function AgentConsole() {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [isCreating, setIsCreating] = useState<boolean>(false);
  const [isEditing, setIsEditing] = useState<boolean>(false);
  const [editAgent, setEditAgent] = useState<Partial<AgentCreate> & { id?: string; status?: string }>(getDefaultAgent());
  const [chatMessages, setChatMessages] = useState<ChatMessage[]>([]);
  const [messageInput, setMessageInput] = useState<string>('');
  const [isSending, setIsSending] = useState<boolean>(false);
  const [serviceStatus, setServiceStatus] = useState<{ status: string; agent_count: number; active_agents: number } | null>(null);

  useEffect(() => {
    loadAgents();
    loadServiceStatus();
  }, []);

  const loadAgents = async () => {
    try {
      setIsLoading(true);
      const response = await listAgents();
      setAgents(response.agents);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load agents');
    } finally {
      setIsLoading(false);
    }
  };

  const loadServiceStatus = async () => {
    try {
      const status = await getAgentServiceStatus();
      setServiceStatus({
        status: status.status,
        agent_count: status.agent_count,
        active_agents: status.active_agents,
      });
    } catch (err) {
      // Service might not be available
    }
  };

  const handleCreateAgent = async () => {
    try {
      setIsLoading(true);
      const newAgent = await createAgent(editAgent as AgentCreate);
      setAgents([...agents, newAgent]);
      setIsCreating(false);
      setEditAgent(getDefaultAgent());
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create agent');
    } finally {
      setIsLoading(false);
    }
  };

  const handleUpdateAgent = async () => {
    if (!selectedAgent) return;
    
    try {
      setIsLoading(true);
      const updateData: AgentUpdate = {
        name: editAgent.name,
        description: editAgent.description,
        model: editAgent.model,
        system_prompt: editAgent.system_prompt,
        temperature: editAgent.temperature,
        max_tokens: editAgent.max_tokens,
        status: editAgent.status as AgentStatus,
      };
      
      const updatedAgent = await updateAgent(selectedAgent.id, updateData);
      setAgents(agents.map(a => a.id === selectedAgent.id ? updatedAgent : a));
      setSelectedAgent(updatedAgent);
      setIsEditing(false);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update agent');
    } finally {
      setIsLoading(false);
    }
  };

  const handleDeleteAgent = async (agentId: string) => {
    if (!window.confirm('Are you sure you want to delete this agent?')) return;
    
    try {
      setIsLoading(true);
      await deleteAgent(agentId);
      setAgents(agents.filter(a => a.id !== agentId));
      if (selectedAgent?.id === agentId) {
        setSelectedAgent(null);
        setChatMessages([]);
      }
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete agent');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSendMessage = async () => {
    if (!selectedAgent || !messageInput.trim()) return;
    
    try {
      setIsSending(true);
      
      const userMessage: ChatMessage = {
        id: 'msg-' + Date.now(),
        role: 'user',
        content: messageInput,
        timestamp: new Date().toISOString(),
      };
      setChatMessages([...chatMessages, userMessage]);
      setMessageInput('');
      
      const response = await sendAgentMessage(selectedAgent.id, { content: messageInput });
      
      const assistantMessage: ChatMessage = {
        id: response.id,
        role: 'assistant',
        content: response.content,
        timestamp: response.timestamp,
      };
      setChatMessages(prev => [...prev, assistantMessage]);
      
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to send message');
    } finally {
      setIsSending(false);
    }
  };

  const handleSelectAgent = (agent: Agent) => {
    setSelectedAgent(agent);
    setChatMessages([]);
    setIsEditing(false);
  };

  const handleNewAgent = () => {
    setIsCreating(true);
    setIsEditing(false);
    setEditAgent(getDefaultAgent());
  };

  const handleEditAgent = (agent: Agent) => {
    setSelectedAgent(agent);
    setIsEditing(true);
    setIsCreating(false);
    setEditAgent({
      id: agent.id,
      name: agent.name,
      description: agent.description || '',
      model: agent.model,
      system_prompt: agent.system_prompt,
      temperature: agent.temperature,
      max_tokens: agent.max_tokens,
      status: agent.status,
    });
  };

  const handleCancelEdit = () => {
    setIsCreating(false);
    setIsEditing(false);
    setEditAgent(getDefaultAgent());
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const name = e.target.name;
    const value = e.target.value;
    setEditAgent(prev => ({ ...prev, [name]: value }));
  };

  const handleModelChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setEditAgent(prev => ({ ...prev, model: e.target.value }));
  };

  const handleStatusChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setEditAgent(prev => ({ ...prev, status: e.target.value as AgentStatus }));
  };

  const handleTemperatureChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setEditAgent(prev => ({ ...prev, temperature: parseFloat(e.target.value) }));
  };

  const handleMaxTokensChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setEditAgent(prev => ({ ...prev, max_tokens: parseInt(e.target.value) }));
  };

  if (isLoading && agents.length === 0) {
    return (
      <div className="agent-console">
        <div className="loading">Loading agents...</div>
      </div>
    );
  }

  return (
    <div className="agent-console">
      <div className="console-header">
        <h2>Agent Console</h2>
        {serviceStatus && (
          <div className="service-status">
            <span className={serviceStatus.status === 'healthy' ? 'status-healthy' : 'status-unhealthy'}>
              {serviceStatus.status}
            </span>
            <span className="agent-count">{serviceStatus.agent_count} agents</span>
          </div>
        )}
      </div>

      {error && (
        <div className="error-message">{error}</div>
      )}

      <div className="console-layout">
        <div className="agent-list">
          <div className="agent-list-header">
            <h3>Agents</h3>
            <button onClick={handleNewAgent} className="btn btn-primary btn-sm">
              + New Agent
            </button>
          </div>
          
          {agents.length === 0 ? (
            <div className="empty-state">No agents found. Create your first agent.</div>
          ) : (
            <ul className="agent-items">
              {agents.map(agent => (
                <li
                  key={agent.id}
                  className={selectedAgent?.id === agent.id ? 'selected' : ''}
                  onClick={() => handleSelectAgent(agent)}
                >
                  <div className="agent-info">
                    <span className="agent-name">{agent.name}</span>
                    <span className={'agent-status ' + getStatusColor(agent.status)}>
                      {getStatusText(agent.status)}
                    </span>
                  </div>
                  <div className="agent-actions">
                    <button onClick={(e) => { e.stopPropagation(); handleEditAgent(agent); }} className="btn-icon">
                      Edit
                    </button>
                    <button onClick={(e) => { e.stopPropagation(); handleDeleteAgent(agent.id); }} className="btn-icon btn-danger">
                      Delete
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </div>

        <div className="console-main">
          {isCreating || isEditing ? (
            <AgentForm
              agent={editAgent}
              isCreating={isCreating}
              isEditing={isEditing}
              onInputChange={handleInputChange}
              onModelChange={handleModelChange}
              onStatusChange={handleStatusChange}
              onTemperatureChange={handleTemperatureChange}
              onMaxTokensChange={handleMaxTokensChange}
              onSubmit={isCreating ? handleCreateAgent : handleUpdateAgent}
              onCancel={handleCancelEdit}
              isLoading={isLoading}
            />
          ) : selectedAgent ? (
            <AgentDetail
              agent={selectedAgent}
              messages={chatMessages}
              messageInput={messageInput}
              onMessageChange={(e) => setMessageInput(e.target.value)}
              onSendMessage={handleSendMessage}
              isSending={isSending}
              onEdit={() => handleEditAgent(selectedAgent)}
            />
          ) : (
            <div className="empty-state">
              <p>Select an agent to view details or create a new one.</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function AgentForm(props: {
  agent: Partial<AgentCreate> & { id?: string; status?: string };
  isCreating: boolean;
  isEditing: boolean;
  onInputChange: (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
  onModelChange: (e: React.ChangeEvent<HTMLSelectElement>) => void;
  onStatusChange: (e: React.ChangeEvent<HTMLSelectElement>) => void;
  onTemperatureChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onMaxTokensChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: () => void;
  onCancel: () => void;
  isLoading: boolean;
}) {
  const agent = props.agent;
  const isCreating = props.isCreating;
  const isEditing = props.isEditing;
  const onInputChange = props.onInputChange;
  const onModelChange = props.onModelChange;
  const onStatusChange = props.onStatusChange;
  const onTemperatureChange = props.onTemperatureChange;
  const onMaxTokensChange = props.onMaxTokensChange;
  const onSubmit = props.onSubmit;
  const onCancel = props.onCancel;
  const isLoading = props.isLoading;

  return (
    <div className="agent-form">
      <h3>{isCreating ? 'Create Agent' : 'Edit Agent'}</h3>
      
      <form onSubmit={(e) => { e.preventDefault(); onSubmit(); }}>
        <div className="form-group">
          <label htmlFor="name">Name</label>
          <input
            type="text"
            id="name"
            name="name"
            value={agent.name || ''}
            onChange={onInputChange}
            required
            placeholder="Agent name"
          />
        </div>

        <div className="form-group">
          <label htmlFor="description">Description</label>
          <textarea
            id="description"
            name="description"
            value={agent.description || ''}
            onChange={onInputChange}
            placeholder="Agent description"
            rows={3}
          />
        </div>

        <div className="form-group">
          <label htmlFor="model">Model</label>
          <select
            id="model"
            name="model"
            value={agent.model || 'mistral_small'}
            onChange={onModelChange}
            required
          >
            <option value="mistral_tiny">Mistral Tiny</option>
            <option value="mistral_small">Mistral Small</option>
            <option value="mistral_medium">Mistral Medium</option>
            <option value="mistral_large">Mistral Large</option>
            <option value="codestral">Codestral</option>
            <option value="mixtral_8x7b">Mixtral 8x7B</option>
            <option value="mixtral_8x22b">Mixtral 8x22B</option>
          </select>
        </div>

        <div className="form-group">
          <label htmlFor="system_prompt">System Prompt</label>
          <textarea
            id="system_prompt"
            name="system_prompt"
            value={agent.system_prompt || ''}
            onChange={onInputChange}
            placeholder="Define the agent system prompt..."
            rows={5}
            required
          />
        </div>

        <div className="form-row">
          <div className="form-group">
            <label htmlFor="temperature">Temperature</label>
            <input
              type="number"
              id="temperature"
              name="temperature"
              value={agent.temperature || 0.7}
              onChange={onTemperatureChange}
              min="0"
              max="2"
              step="0.1"
            />
          </div>

          <div className="form-group">
            <label htmlFor="max_tokens">Max Tokens</label>
            <input
              type="number"
              id="max_tokens"
              name="max_tokens"
              value={agent.max_tokens || 4096}
              onChange={onMaxTokensChange}
              min="1"
              max="32768"
            />
          </div>
        </div>

        {isEditing && (
          <div className="form-group">
            <label htmlFor="status">Status</label>
            <select
              id="status"
              name="status"
              value={agent.status || 'active'}
              onChange={onStatusChange}
            >
              <option value="active">Active</option>
              <option value="inactive">Inactive</option>
              <option value="maintenance">Maintenance</option>
              <option value="disabled">Disabled</option>
            </select>
          </div>
        )}

        <div className="form-actions">
          <button type="button" onClick={onCancel} className="btn btn-secondary">
            Cancel
          </button>
          <button type="submit" disabled={isLoading} className="btn btn-primary">
            {isLoading ? 'Saving...' : (isCreating ? 'Create' : 'Update')}
          </button>
        </div>
      </form>
    </div>
  );
}

function AgentDetail(props: {
  agent: Agent;
  messages: ChatMessage[];
  messageInput: string;
  onMessageChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSendMessage: () => void;
  isSending: boolean;
  onEdit: () => void;
}) {
  const agent = props.agent;
  const messages = props.messages;
  const messageInput = props.messageInput;
  const onMessageChange = props.onMessageChange;
  const onSendMessage = props.onSendMessage;
  const isSending = props.isSending;
  const onEdit = props.onEdit;

  return (
    <div className="agent-detail">
      <div className="agent-header">
        <h3>{agent.name}</h3>
        <div className="agent-meta">
          <span className={'status-badge ' + getStatusColor(agent.status)}>
            {getStatusText(agent.status)}
          </span>
          <span className="model-badge">{getModelDisplayName(agent.model)}</span>
        </div>
        <button onClick={onEdit} className="btn btn-secondary btn-sm">
          Edit
        </button>
      </div>

      <div className="agent-info">
        <p className="agent-description">{agent.description || 'No description provided.'}</p>
        <div className="agent-settings">
          <span>Temperature: {agent.temperature}</span>
          <span>Max Tokens: {agent.max_tokens}</span>
        </div>
      </div>

      <div className="system-prompt">
        <h4>System Prompt</h4>
        <pre>{agent.system_prompt}</pre>
      </div>

      <div className="chat-console">
        <h4>Chat</h4>
        <div className="chat-messages">
          {messages.length === 0 ? (
            <div className="empty-chat">Start a conversation with {agent.name}</div>
          ) : (
            messages.map(message => (
              <div key={message.id} className={'chat-message ' + message.role}>
                <div className="message-header">
                  <span className="message-role">{message.role}</span>
                  <span className="message-time">{new Date(message.timestamp).toLocaleTimeString()}</span>
                </div>
                <div className="message-content">{message.content}</div>
              </div>
            ))
          )}
        </div>

        <div className="chat-input">
          <input
            type="text"
            value={messageInput}
            onChange={onMessageChange}
            onKeyPress={(e) => e.key === 'Enter' && !isSending && onSendMessage()}
            placeholder="Type your message..."
            disabled={isSending}
          />
          <button onClick={onSendMessage} disabled={isSending || !messageInput.trim()} className="btn btn-primary">
            {isSending ? 'Sending...' : 'Send'}
          </button>
        </div>
      </div>
    </div>
  );
}