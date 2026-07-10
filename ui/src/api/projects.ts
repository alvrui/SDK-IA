import { API_CONFIG, fetchWithTimeout } from './config';

export interface Project {
  id: string;
  name: string;
  description: string;
  created_at: string;
  updated_at: string;
  status: string;
}

export interface ProjectCreate {
  name: string;
  description: string;
}

export interface ProjectsResponse {
  projects: Project[];
}

export async function listProjects(): Promise<ProjectsResponse> {
  const response = await fetchWithTimeout(
    `${API_CONFIG.RUST_BACKEND}/projects`,
    { method: 'GET' }
  );
  if (!response.ok) {
    throw new Error('Failed to fetch projects');
  }
  return response.json();
}

export async function createProject(project: ProjectCreate): Promise<Project> {
  const response = await fetchWithTimeout(
    `${API_CONFIG.RUST_BACKEND}/projects`,
    {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(project)
    }
  );
  if (!response.ok) {
    throw new Error('Failed to create project');
  }
  return response.json();
}

export async function getProject(id: string): Promise<Project> {
  const response = await fetchWithTimeout(
    `${API_CONFIG.RUST_BACKEND}/projects/${id}`,
    { method: 'GET' }
  );
  if (!response.ok) {
    throw new Error('Failed to fetch project');
  }
  return response.json();
}

export async function updateProject(id: string, project: Partial<ProjectCreate>): Promise<Project> {
  const response = await fetchWithTimeout(
    `${API_CONFIG.RUST_BACKEND}/projects/${id}`,
    {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(project)
    }
  );
  if (!response.ok) {
    throw new Error('Failed to update project');
  }
  return response.json();
}

export async function deleteProject(id: string): Promise<void> {
  const response = await fetchWithTimeout(
    `${API_CONFIG.RUST_BACKEND}/projects/${id}`,
    { method: 'DELETE' }
  );
  if (!response.ok) {
    throw new Error('Failed to delete project');
  }
}
