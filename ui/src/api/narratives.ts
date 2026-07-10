
// API client for Narrative management
// Connects to Rust backend on port 9090

import { API_CONFIG, fetchWithTimeout } from './config';

export interface StoryElement {
  id: string;
  narrative_id: string;
  element_type: string;
  hollywood_element_id: string;
  name: string;
  description: string;
  attributes: Record<string, string>;
  created_at: string;
  updated_at: string;
}

export interface Narrative {
  id: string;
  project_id: string;
  title: string;
  synopsis: string;
  version: string;
  status: string;
  compatibility_score: number;
  theme_ids: string[];
  metadata: Record<string, string>;
  created_at: string;
  updated_at: string;
  story_elements: StoryElement[];
}

export interface NarrativeCreate {
  project_id: string;
  title: string;
  synopsis: string;
  theme_ids?: string[];
  metadata?: Record<string, string>;
}

export interface NarrativeUpdate {
  title?: string;
  synopsis?: string;
  status?: string;
  theme_ids?: string[];
  metadata?: Record<string, string>;
}

export interface NarrativesResponse {
  status: string;
  data: Narrative[];
  meta?: {
    count: number;
    project_id: string;
  };
}

export interface HollywoodElement {
  id: string;
  name: string;
  type: string;
  description: string;
}

export interface CompatibilityResult {
  element1: string;
  element2: string;
  compatibility_score: number;
  compatible: boolean;
}

const NARRATIVES_API_BASE = API_CONFIG.RUST_BACKEND;

export async function listNarratives(projectId?: string): Promise<{ narratives: Narrative[] }> {
  let url = NARRATIVES_API_BASE + '/narratives';
  if (projectId) {
    url += '?project_id=' + projectId;
  }
  
  const response = await fetchWithTimeout(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch narratives: ${response.statusText}`);
  }
  const data = await response.json();
  if (data.status === 'success') {
    return { narratives: data.data || [] };
  }
  throw new Error(data.error || 'Failed to fetch narratives');
}

export async function createNarrative(narrative: NarrativeCreate): Promise<Narrative> {
  const response = await fetchWithTimeout(NARRATIVES_API_BASE + '/narratives', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(narrative),
  });
  if (!response.ok) {
    throw new Error(`Failed to create narrative: ${response.statusText}`);
  }
  const data = await response.json();
  if (data.status === 'success') {
    return data.data;
  }
  throw new Error(data.error || 'Failed to create narrative');
}

export async function getNarrative(id: string): Promise<Narrative> {
  const response = await fetchWithTimeout(NARRATIVES_API_BASE + '/narratives/' + id);
  if (!response.ok) {
    throw new Error(`Failed to fetch narrative: ${response.statusText}`);
  }
  const data = await response.json();
  if (data.status === 'success') {
    const narrativeData = data.data;
    // Handle both formats: {narrative, story_elements} or just narrative
    if (narrativeData.narrative && narrativeData.story_elements) {
      return {
        ...narrativeData.narrative,
        story_elements: narrativeData.story_elements
      };
    }
    return narrativeData;
  }
  throw new Error(data.error || 'Failed to fetch narrative');
}

export async function updateNarrative(id: string, narrative: NarrativeUpdate): Promise<Narrative> {
  const response = await fetchWithTimeout(NARRATIVES_API_BASE + '/narratives/' + id, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(narrative),
  });
  if (!response.ok) {
    throw new Error(`Failed to update narrative: ${response.statusText}`);
  }
  const data = await response.json();
  if (data.status === 'success') {
    return data.data;
  }
  throw new Error(data.error || 'Failed to update narrative');
}

export async function deleteNarrative(id: string): Promise<void> {
  const response = await fetchWithTimeout(NARRATIVES_API_BASE + '/narratives/' + id, {
    method: 'DELETE',
  });
  if (!response.ok) {
    throw new Error(`Failed to delete narrative: ${response.statusText}`);
  }
}

// Hollywood Animal endpoints
export async function getHollywoodElements(): Promise<HollywoodElement[]> {
  const response = await fetchWithTimeout(NARRATIVES_API_BASE + '/hollywood-animal/elements');
  if (!response.ok) {
    throw new Error(`Failed to fetch Hollywood elements: ${response.statusText}`);
  }
  return response.json();
}

export async function checkCompatibility(element1: string, element2: string): Promise<CompatibilityResult> {
  const response = await fetchWithTimeout(
    NARRATIVES_API_BASE + '/hollywood-animal/compatibility/' + element1 + '/' + element2
  );
  if (!response.ok) {
    throw new Error(`Failed to check compatibility: ${response.statusText}`);
  }
  return response.json();
}

export async function getCompatibilityMatrix(): Promise<any> {
  const response = await fetchWithTimeout(NARRATIVES_API_BASE + '/hollywood-animal/matrix');
  if (!response.ok) {
    throw new Error(`Failed to fetch compatibility matrix: ${response.statusText}`);
  }
  return response.json();
}
