
// API client for Project management
// Connects to Rust backend on port 9090

import { API_CONFIG, fetchWithTimeout } from './config';

export interface Project {
  id: string;
  name: string;
  description: string;
  author: string;
  version: string;
  status: string;
  created_at: string;
  updated_at: string;
  tags: string[];
  metadata: Record<string, string>;
}

export interface ProjectCreate {
  name: string;
  description: string;
  author: string;
  tags?: string[];
  metadata?: Record<string, string>;
}

export interface ProjectUpdate {
  name?: string;
  description?: string;
  author?: string;
  status?: string;
  tags?: string[];
  metadata?: Record<string, string>;
}

export interface ProjectsResponse {
  status: string;
  data: Project[];
  meta?: {
    page: number;
    page_size: number;
    total: number;
    total_pages: number;
  };
}

const PROJECTS_API_BASE = API_CONFIG.RUST_BACKEND;

// Helper to extract projects from response
export async function extractProjects(response: Response): Promise<Project[]> {
  const data = await response.json();
  if (data.status === 'success') {
    return data.data || [];
  }
  throw new Error(data.error || 'Failed to fetch projects');
}

export async function listProjects(): Promise<{ projects: Project[] }> {
  const response = await fetchWithTimeout(PROJECTS_API_BASE + '/projects');
  if (!response.ok) {
    throw new Error(`Failed to fetch projects: ${response.statusText}`);
  }
  const data = await response.json();
  if (data.status === 'success') {
    return { projects: data.data || [] };
  }
  throw new Error(data.error || 'Failed to fetch projects');
}

export async function createProject(project: ProjectCreate): Promise<Project> {
  const response = await fetchWithTimeout(PROJECTS_API_BASE + '/projects', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(project),
  });
  if (!response.ok) {
    throw new Error(`Failed to create project: ${response.statusText}`);
  }
  const data = await response.json();
  if (data.status === 'success') {
    return data.data;
  }
  throw new Error(data.error || 'Failed to create project');
}

export async function getProject(id: string): Promise<Project> {
  const response = await fetchWithTimeout(PROJECTS_API_BASE + '/projects/' + id);
  if (!response.ok) {
    throw new Error(`Failed to fetch project: ${response.statusText}`);
  }
  const data = await response.json();
  if (data.status === 'success') {
    return data.data;
  }
  throw new Error(data.error || 'Failed to fetch project');
}

export async function updateProject(id: string, project: ProjectUpdate): Promise<Project> {
  const response = await fetchWithTimeout(PROJECTS_API_BASE + '/projects/' + id, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(project),
  });
  if (!response.ok) {
    throw new Error(`Failed to update project: ${response.statusText}`);
  }
  const data = await response.json();
  if (data.status === 'success') {
    return data.data;
  }
  throw new Error(data.error || 'Failed to update project');
}

export async function deleteProject(id: string): Promise<void> {
  const response = await fetchWithTimeout(PROJECTS_API_BASE + '/projects/' + id, {
    method: 'DELETE',
  });
  if (!response.ok) {
    throw new Error(`Failed to delete project: ${response.statusText}`);
  }
}
