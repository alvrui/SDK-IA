import { useState, useEffect, useCallback } from 'react';
import apiClient, { Agent, ApiResponse } from '../api/client';

export const useAgents = () => {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchAgents = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.getAgents();
      if (response.status === 'success') setAgents(response.data || []);
      else setError(response.error?.message || 'Failed to fetch agents');
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); }
    finally { setLoading(false); }
  }, []);

  const createAgent = useCallback(async (agentData: Omit<Agent, 'id' | 'created_at'>) => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.createAgent(agentData);
      if (response.status === 'success') { setAgents(prev => [response.data!, ...prev]); return response.data; }
      else { setError(response.error?.message || 'Failed to create agent'); return null; }
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); return null; }
    finally { setLoading(false); }
  }, []);

  const updateAgent = useCallback(async (id: string, agentData: Partial<Agent>) => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.updateAgent(id, agentData);
      if (response.status === 'success') { setAgents(prev => prev.map(a => a.id === id ? response.data! : a)); return response.data; }
      else { setError(response.error?.message || 'Failed to update agent'); return null; }
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); return null; }
    finally { setLoading(false); }
  }, []);

  const deleteAgent = useCallback(async (id: string) => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.deleteAgent(id);
      if (response.status === 'success') { setAgents(prev => prev.filter(a => a.id !== id)); return true; }
      else { setError(response.error?.message || 'Failed to delete agent'); return false; }
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); return false; }
    finally { setLoading(false); }
  }, []);

  useEffect(() => { fetchAgents(); }, [fetchAgents]);
  return { agents, loading, error, fetchAgents, createAgent, updateAgent, deleteAgent };
};

export const useAgent = (id: string) => {
  const [agent, setAgent] = useState<Agent | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchAgent = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.getAgent(id);
      if (response.status === 'success') setAgent(response.data || null);
      else setError(response.error?.message || 'Failed to fetch agent');
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); }
    finally { setLoading(false); }
  }, [id]);

  useEffect(() => { if (id) fetchAgent(); }, [id, fetchAgent]);
  return { agent, loading, error, fetchAgent };
};