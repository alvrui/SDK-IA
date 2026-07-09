// Type definitions for Agent management

export interface AgentModel {
  mistral_tiny: string;
  mistral_small: string;
  mistral_medium: string;
  mistral_large: string;
  codestral: string;
  mixtral_8x7b: string;
  mixtral_8x22b: string;
  custom: string;
}

export const AGENT_MODELS: AgentModel = {
  mistral_tiny: 'mistral-tiny-latest',
  mistral_small: 'mistral-small-latest',
  mistral_medium: 'mistral-medium-latest',
  mistral_large: 'mistral-large-latest',
  codestral: 'codestral-latest',
  mixtral_8x7b: 'mixtral-8x7b-latest',
  mixtral_8x22b: 'mixtral-8x22b-latest',
  custom: 'custom',
};

export type AgentModelType = keyof typeof AGENT_MODELS;

export interface Agent {
  id: string;
  name: string;
  description: string | null;
  model: AgentModelType | string;
  system_prompt: string;
  temperature: number;
  max_tokens: number;
  status: AgentStatus;
  created_at: string;
  updated_at: string;
  metadata: Record<string, unknown> | null;
}

export type AgentStatus = 'active' | 'inactive' | 'maintenance' | 'disabled';

export interface AgentCreate {
  name: string;
  description?: string;
  model?: AgentModelType | string;
  system_prompt: string;
  temperature?: number;
  max_tokens?: number;
}

export interface AgentUpdate {
  name?: string;
  description?: string;
  model?: AgentModelType | string;
  system_prompt?: string;
  temperature?: number;
  max_tokens?: number;
  status?: AgentStatus;
  metadata?: Record<string, unknown>;
}

export interface AgentMessage {
  id: string;
  agent_id: string;
  conversation_id: string | null;
  content: string;
  role: 'user' | 'assistant';
  timestamp: string;
  metadata: Record<string, unknown> | null;
}

export interface AgentMessageCreate {
  content: string;
  conversation_id?: string;
}

export interface AgentConversation {
  id: string;
  agent_id: string;
  title?: string;
  created_at: string;
  updated_at: string;
  message_count: number;
  metadata: Record<string, unknown> | null;
}

export interface AgentListResponse {
  agents: Agent[];
  total: number;
}

export interface AgentServiceStatus {
  status: string;
  agent_count: number;
  active_agents: number;
  version: string;
}

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: string;
  loading?: boolean;
}

export interface Conversation {
  id: string;
  agent_id: string;
  title: string;
  messages: ChatMessage[];
  created_at: string;
  updated_at: string;
}