import { API_CONFIG, fetchWithTimeout } from './config';

export interface Narrative {
  id: string;
  name: string;
  description: string;
  project_id: string;
  story_elements: any[];
  created_at: string;
  updated_at: string;
}

export interface NarrativeCreate {
  name: string;
  description: string;
  project_id: string;
}

export interface NarrativesResponse {
  narratives: Narrative[];
}

export async function listNarratives(projectId?: string): Promise<NarrativesResponse> {
  const url = projectId
    ? `${API_CONFIG.RUST_BACKEND}/narratives?project_id=${projectId}`
    : `${API_CONFIG.RUST_BACKEND}/narratives`;

  const response = await fetchWithTimeout(url, { method: 'GET' });
  if (!response.ok) {
    throw new Error('Failed to fetch narratives');
  }
  return response.json();
}

export async function createNarrative(narrative: NarrativeCreate): Promise<Narrative> {
  const response = await fetchWithTimeout(
    `${API_CONFIG.RUST_BACKEND}/narratives`,
    {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(narrative)
    }
  );
  if (!response.ok) {
    throw new Error('Failed to create narrative');
  }
  return response.json();
}

export async function getNarrative(id: string): Promise<Narrative> {
  const response = await fetchWithTimeout(
    `${API_CONFIG.RUST_BACKEND}/narratives/${id}`,
    { method: 'GET' }
  );
  if (!response.ok) {
    throw new Error('Failed to fetch narrative');
  }
  return response.json();
}

// Hollywood Animal endpoints
export async function getHollywoodElements() {
  const response = await fetchWithTimeout(
    `${API_CONFIG.RUST_BACKEND}/api/v1/internal/hollywood-animal/elements`,
    { method: 'GET' }
  );
  if (!response.ok) {
    throw new Error('Failed to fetch Hollywood elements');
  }
  return response.json();
}

export async function checkCompatibility(element1: string, element2: string) {
  const response = await fetchWithTimeout(
    `${API_CONFIG.RUST_BACKEND}/api/v1/internal/hollywood-animal/compatibility/${element1}/${element2}`,
    { method: 'GET' }
  );
  if (!response.ok) {
    throw new Error('Failed to check compatibility');
  }
  return response.json();
}
