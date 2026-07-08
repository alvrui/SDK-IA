const BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:9090';

interface ApiResponse<T> {
  status: 'success' | 'error';
  data?: T;
  error?: { code: string; message: string; details?: Record<string, unknown> };
  meta?: Record<string, unknown>;
}

interface Project {
  id: string; name: string; description: string;
  created_at: string; updated_at: string; status: 'draft' | 'active' | 'archived';
}
interface Agent {
  id: string; name: string; model: string; description: string;
  status: 'active' | 'inactive' | 'error'; created_at: string;
}
interface Narrative {
  id: string; project_id: string; title: string; content: string;
  status: 'draft' | 'generated' | 'validated'; created_at: string;
}
interface StoryElement {
  id: string; project_id: string; narrative_id: string;
  element_type: 'character' | 'event' | 'location' | 'object';
  name: string; description: string; properties: Record<string, unknown>; created_at: string;
}
interface Event {
  id: string; project_id: string; narrative_id: string;
  title: string; description: string; timestamp: string;
  location: string; participants: string[];
  status: 'draft' | 'generated' | 'validated'; created_at: string;
}
interface Message {
  id: string; agent_id: string; conversation_id: string;
  content: string; role: 'user' | 'assistant' | 'system';
  timestamp: string; metadata: Record<string, unknown>;
}
interface Conversation {
  id: string; agent_id: string; project_id: string;
  title: string; messages: Message[]; created_at: string; updated_at: string;
}
interface GenerationRequest {
  project_id: string; narrative_id?: string;
  element_type?: 'narrative' | 'story-element' | 'event';
  prompt: string; parameters: Record<string, unknown>;
}
interface GenerationResult {
  id: string; project_id: string; content: string;
  element_type: 'narrative' | 'story-element' | 'event';
  status: 'pending' | 'processing' | 'completed' | 'failed';
  created_at: string; completed_at?: string;
}

const apiClient = {
  baseURL: BASE_URL,
  
  async request<T>(endpoint: string, options?: RequestInit): Promise<ApiResponse<T>> {
    const url = `${this.baseURL}${endpoint}`;
    try {
      const response = await fetch(url, {
        ...options,
        headers: { 'Content-Type': 'application/json', ...options?.headers }
      });
      const data = await response.json();
      if (!response.ok) throw new Error(data.error?.message || `HTTP error! status: ${response.status}`);
      return data as ApiResponse<T>;
    } catch (error) {
      return {
        status: 'error',
        error: { code: 'NETWORK_ERROR', message: error instanceof Error ? error.message : 'Unknown error' }
      };
    }
  },

  async getProjects(): Promise<ApiResponse<Project[]>> { return this.request('/api/v1/internal/projects'); },
  async getProject(id: string): Promise<ApiResponse<Project>> { return this.request(`/api/v1/internal/projects/${id}`); },
  async createProject(project: Omit<Project, 'id' | 'created_at' | 'updated_at'>): Promise<ApiResponse<Project>> {
    return this.request('/api/v1/internal/projects', { method: 'POST', body: JSON.stringify(project) });
  },
  async updateProject(id: string, project: Partial<Project>): Promise<ApiResponse<Project>> {
    return this.request(`/api/v1/internal/projects/${id}`, { method: 'PUT', body: JSON.stringify(project) });
  },
  async deleteProject(id: string): Promise<ApiResponse<void>> {
    return this.request(`/api/v1/internal/projects/${id}`, { method: 'DELETE' });
  },
  async getAgents(): Promise<ApiResponse<Agent[]>> { return this.request('/api/v1/internal/agents'); },
  async getAgent(id: string): Promise<ApiResponse<Agent>> { return this.request(`/api/v1/internal/agents/${id}`); },
  async createAgent(agent: Omit<Agent, 'id' | 'created_at'>): Promise<ApiResponse<Agent>> {
    return this.request('/api/v1/internal/agents', { method: 'POST', body: JSON.stringify(agent) });
  },
  async updateAgent(id: string, agent: Partial<Agent>): Promise<ApiResponse<Agent>> {
    return this.request(`/api/v1/internal/agents/${id}`, { method: 'PUT', body: JSON.stringify(agent) });
  },
  async deleteAgent(id: string): Promise<ApiResponse<void>> {
    return this.request(`/api/v1/internal/agents/${id}`, { method: 'DELETE' });
  },
  async getNarratives(projectId: string): Promise<ApiResponse<Narrative[]>> {
    return this.request(`/api/v1/internal/projects/${projectId}/narratives`);
  },
  async createNarrative(projectId: string, narrative: Omit<Narrative, 'id' | 'project_id' | 'created_at'>): Promise<ApiResponse<Narrative>> {
    return this.request(`/api/v1/internal/projects/${projectId}/narratives`, { method: 'POST', body: JSON.stringify(narrative) });
  },
  async getStoryElements(projectId: string): Promise<ApiResponse<StoryElement[]>> {
    return this.request(`/api/v1/internal/projects/${projectId}/story-elements`);
  },
  async createStoryElement(projectId: string, element: Omit<StoryElement, 'id' | 'project_id' | 'created_at'>): Promise<ApiResponse<StoryElement>> {
    return this.request(`/api/v1/internal/projects/${projectId}/story-elements`, { method: 'POST', body: JSON.stringify(element) });
  },
  async getEvents(projectId: string): Promise<ApiResponse<Event[]>> {
    return this.request(`/api/v1/internal/projects/${projectId}/events`);
  },
  async createEvent(projectId: string, event: Omit<Event, 'id' | 'project_id' | 'created_at'>): Promise<ApiResponse<Event>> {
    return this.request(`/api/v1/internal/projects/${projectId}/events`, { method: 'POST', body: JSON.stringify(event) });
  },
  async generateContent(request: GenerationRequest): Promise<ApiResponse<GenerationResult>> {
    return this.request('/api/v1/internal/generate', { method: 'POST', body: JSON.stringify(request) });
  },
  async getGenerationStatus(generationId: string): Promise<ApiResponse<GenerationResult>> {
    return this.request(`/api/v1/internal/generate/${generationId}`);
  },
  async getConversations(agentId?: string, projectId?: string): Promise<ApiResponse<Conversation[]>> {
    const params = new URLSearchParams();
    if (agentId) params.append('agent_id', agentId);
    if (projectId) params.append('project_id', projectId);
    const query = params.toString() ? `?${params.toString()}` : '';
    return this.request(`/api/v1/internal/conversations${query}`);
  },
  async getConversation(conversationId: string): Promise<ApiResponse<Conversation>> {
    return this.request(`/api/v1/internal/conversations/${conversationId}`);
  },
  async createConversation(agentId: string, projectId: string, title: string): Promise<ApiResponse<Conversation>> {
    return this.request('/api/v1/internal/conversations', { method: 'POST', body: JSON.stringify({ agent_id: agentId, project_id: projectId, title }) });
  },
  async sendMessage(conversationId: string, content: string, role: 'user' | 'system' = 'user'): Promise<ApiResponse<Message>> {
    return this.request(`/api/v1/internal/conversations/${conversationId}/messages`, { method: 'POST', body: JSON.stringify({ content, role }) });
  },
  async healthCheck(): Promise<ApiResponse<{ status: string; version: string; timestamp: string }>> {
    return this.request('/api/v1/internal/health');
  }
};

export type { ApiResponse, Project, Agent, Narrative, StoryElement, Event, Message, Conversation, GenerationRequest, GenerationResult };
export default apiClient;