// API client for Agent management

import { Agent, AgentCreate, AgentUpdate, AgentMessage, AgentMessageCreate, AgentConversation, AgentListResponse, AgentServiceStatus } from '../types/agent';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:9000';

// Helper to handle API errors
export async function handleApiError(response: Response): Promise<never> {
  const errorData = await response.json().catch(() => ({}));
  const errorMessage = errorData.message || errorData.error || 'Unknown error';
  throw new Error(errorMessage);
}

// Agent CRUD Operations
export async function listAgents(): Promise<AgentListResponse> {
  const response = await fetch(API_BASE_URL + '/api/v1/internal/agents');
  if (!response.ok) {
    await handleApiError(response);
  }
  return response.json();
}

export async function createAgent(agentData: AgentCreate): Promise<Agent> {
  const response = await fetch(API_BASE_URL + '/api/v1/internal/agents', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(agentData),
  });
  if (!response.ok) {
    await handleApiError(response);
  }
  return response.json();
}

export async function getAgent(agentId: string): Promise<Agent> {
  const response = await fetch(API_BASE_URL + '/api/v1/internal/agents/' + agentId);
  if (!response.ok) {
    await handleApiError(response);
  }
  return response.json();
}

export async function updateAgent(agentId: string, agentData: AgentUpdate): Promise<Agent> {
  const response = await fetch(API_BASE_URL + '/api/v1/internal/agents/' + agentId, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(agentData),
  });
  if (!response.ok) {
    await handleApiError(response);
  }
  return response.json();
}

export async function deleteAgent(agentId: string): Promise<void> {
  const response = await fetch(API_BASE_URL + '/api/v1/internal/agents/' + agentId, {
    method: 'DELETE',
  });
  if (!response.ok) {
    await handleApiError(response);
  }
}

// Agent Message Operations
export async function sendAgentMessage(agentId: string, messageData: AgentMessageCreate): Promise<AgentMessage> {
  const response = await fetch(API_BASE_URL + '/api/v1/internal/agents/' + agentId + '/messages', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(messageData),
  });
  if (!response.ok) {
    await handleApiError(response);
  }
  return response.json();
}

// Agent Conversation Operations
export async function listAgentConversations(agentId: string): Promise<AgentConversation[]> {
  const response = await fetch(API_BASE_URL + '/api/v1/internal/agents/' + agentId + '/conversations');
  if (!response.ok) {
    await handleApiError(response);
  }
  return response.json();
}

// Service Status
export async function getAgentServiceStatus(): Promise<AgentServiceStatus> {
  const response = await fetch(API_BASE_URL + '/api/v1/internal/agents/status');
  if (!response.ok) {
    await handleApiError(response);
  }
  return response.json();
}

// Helper Functions
export function getDefaultAgent(): AgentCreate {
  return {
    name: 'New Agent',
    description: 'A helpful AI assistant',
    model: 'mistral_small',
    system_prompt: 'You are a helpful AI assistant. Provide accurate and helpful responses.',
    temperature: 0.7,
    max_tokens: 4096,
  };
}

export function getModelDisplayName(model: string): string {
  const displayNames: Record<string, string> = {
    'mistral-tiny-latest': 'Mistral Tiny',
    'mistral-small-latest': 'Mistral Small',
    'mistral-medium-latest': 'Mistral Medium',
    'mistral-large-latest': 'Mistral Large',
    'codestral-latest': 'Codestral',
    'mixtral-8x7b-latest': 'Mixtral 8x7B',
    'mixtral-8x22b-latest': 'Mixtral 8x22B',
  };
  return displayNames[model] || model;
}

export function getStatusColor(status: string): string {
  const colors: Record<string, string> = {
    active: 'bg-green-500',
    inactive: 'bg-gray-500',
    maintenance: 'bg-yellow-500',
    disabled: 'bg-red-500',
  };
  return colors[status] || 'bg-gray-500';
}

export function getStatusText(status: string): string {
  const texts: Record<string, string> = {
    active: 'Active',
    inactive: 'Inactive',
    maintenance: 'Maintenance',
    disabled: 'Disabled',
  };
  return texts[status] || status;
}