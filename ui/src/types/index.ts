export type { Project, Agent, Narrative, StoryElement, Event, Message, Conversation, GenerationRequest, GenerationResult, ApiResponse } from '../api/client';

export interface SelectOption { value: string; label: string; }
export interface TabItem { id: string; label: string; icon?: React.ReactNode; }

export interface ToastMessage {
  id: string;
  type: 'success' | 'error' | 'info' | 'warning';
  title: string;
  message: string;
  duration?: number;
}

export interface ProjectFormData { name: string; description: string; status: 'draft' | 'active' | 'archived'; }
export interface AgentFormData { name: string; model: string; description: string; }
export interface GenerationFormData { project_id: string; narrative_id?: string; element_type: 'narrative' | 'story-element' | 'event'; prompt: string; temperature?: number; max_tokens?: number; }