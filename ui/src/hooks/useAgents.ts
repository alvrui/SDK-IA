import { useState, useEffect, useCallback } from 'react';
import {
  listAgents,
  createAgent,
  updateAgent,
  deleteAgent,
  sendAgentMessage,
  getAgentServiceStatus,
  Agent,
  AgentCreate,
  AgentUpdate,
  ChatMessage
} from '../api/agents';

export function useAgents() {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [serviceStatus, setServiceStatus] = useState<any>(null);

  const fetchAgents = useCallback(async () => {
    try {
      setLoading(true);
      console.log('[useAgents] Fetching agents from:', process.env.NODE_ENV === 'development' ? 'http://localhost:9000/api/v1/agents' : '/api/v1/agents');
      const response = await listAgents();
      console.log('[useAgents] Response:', response);
      setAgents(response.agents || []);
      setError(null);
    } catch (err) {
      console.error('[useAgents] Error:', err);
      setError(err instanceof Error ? err.message : 'Failed to load agents');
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchServiceStatus = useCallback(async () => {
    try {
      const status = await getAgentServiceStatus();
      console.log('[useAgents] Service status:', status);
      setServiceStatus(status);
    } catch (err) {
      console.error('[useAgents] Service status error:', err);
    }
  }, []);

  useEffect(() => {
    fetchAgents();
    fetchServiceStatus();
  }, [fetchAgents, fetchServiceStatus]);

  const createAgentWrapper = async (agent: AgentCreate) => {
    try {
      console.log('[useAgents] Creating agent:', agent);
      const newAgent = await createAgent(agent);
      console.log('[useAgents] Created:', newAgent);
      setAgents([...agents, newAgent]);
      return newAgent;
    } catch (err) {
      console.error('[useAgents] Create error:', err);
      setError(err instanceof Error ? err.message : 'Failed to create agent');
      throw err;
    }
  };

  const updateAgentWrapper = async (id: string, agent: AgentUpdate) => {
    try {
      console.log('[useAgents] Updating agent:', id, agent);
      const updatedAgent = await updateAgent(id, agent);
      console.log('[useAgents] Updated:', updatedAgent);
      setAgents(agents.map(a => a.id === id ? updatedAgent : a));
      return updatedAgent;
    } catch (err) {
      console.error('[useAgents] Update error:', err);
      setError(err instanceof Error ? err.message : 'Failed to update agent');
      throw err;
    }
  };

  const deleteAgentWrapper = async (id: string) => {
    try {
      console.log('[useAgents] Deleting agent:', id);
      await deleteAgent(id);
      setAgents(agents.filter(a => a.id !== id));
    } catch (err) {
      console.error('[useAgents] Delete error:', err);
      setError(err instanceof Error ? err.message : 'Failed to delete agent');
      throw err;
    }
  };

  const sendMessageWrapper = async (agentId: string, message: { content: string; conversation_id?: string }): Promise<ChatMessage> => {
    try {
      console.log('[useAgents] Sending message to:', agentId, message);
      return await sendAgentMessage(agentId, message);
    } catch (err) {
      console.error('[useAgents] Send message error:', err);
      setError(err instanceof Error ? err.message : 'Failed to send message');
      throw err;
    }
  };

  return {
    agents,
    loading,
    error,
    serviceStatus,
    fetchAgents,
    createAgent: createAgentWrapper,
    updateAgent: updateAgentWrapper,
    deleteAgent: deleteAgentWrapper,
    sendAgentMessage: sendMessageWrapper
  };
}