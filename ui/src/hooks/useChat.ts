import { useState, useCallback } from 'react';
import apiClient, { Conversation, Message, ApiResponse } from '../api/client';

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: string;
  agentId?: string;
}

export const useChat = (conversationId: string) => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [sending, setSending] = useState<boolean>(false);

  const fetchMessages = useCallback(async () => {
    if (!conversationId) return;
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.getConversation(conversationId);
      if (response.status === 'success' && response.data) {
        const chatMessages: ChatMessage[] = response.data.messages.map(msg => ({
          id: msg.id, role: msg.role, content: msg.content,
          timestamp: msg.timestamp, agentId: msg.agent_id
        }));
        setMessages(chatMessages);
      } else setError(response.error?.message || 'Failed to fetch messages');
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); }
    finally { setLoading(false); }
  }, [conversationId]);

  const sendMessage = useCallback(async (content: string, role: 'user' | 'system' = 'user') => {
    if (!conversationId || !content.trim()) return null;
    setSending(true);
    setError(null);
    const optimisticMessage: ChatMessage = {
      id: `temp-${Date.now()}`,
      role,
      content,
      timestamp: new Date().toISOString()
    };
    setMessages(prev => [...prev, optimisticMessage]);
    try {
      const response = await apiClient.sendMessage(conversationId, content, role);
      if (response.status === 'success' && response.data) {
        setMessages(prev => prev.map(m => m.id === optimisticMessage.id ? {
          id: response.data.id,
          role: response.data.role,
          content: response.data.content,
          timestamp: response.data.timestamp,
          agentId: response.data.agent_id
        } : m));
        return response.data;
      } else {
        setError(response.error?.message || 'Failed to send message');
        setMessages(prev => prev.filter(m => m.id !== optimisticMessage.id));
        return null;
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
      setMessages(prev => prev.filter(m => m.id !== optimisticMessage.id));
      return null;
    } finally { setSending(false); }
  }, [conversationId]);

  const clearMessages = useCallback(() => { setMessages([]); }, []);
  return { messages, loading, error, sending, fetchMessages, sendMessage, clearMessages };
};

export const useConversations = (agentId?: string, projectId?: string) => {
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchConversations = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.getConversations(agentId, projectId);
      if (response.status === 'success') setConversations(response.data || []);
      else setError(response.error?.message || 'Failed to fetch conversations');
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); }
    finally { setLoading(false); }
  }, [agentId, projectId]);

  const createConversation = useCallback(async (agentId: string, projectId: string, title: string) => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.createConversation(agentId, projectId, title);
      if (response.status === 'success') { setConversations(prev => [response.data!, ...prev]); return response.data; }
      else { setError(response.error?.message || 'Failed to create conversation'); return null; }
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); return null; }
    finally { setLoading(false); }
  }, []);

  useEffect(() => { fetchConversations(); }, [fetchConversations]);
  return { conversations, loading, error, fetchConversations, createConversation };
};