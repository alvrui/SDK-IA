import { useState, useCallback } from 'react';
import apiClient, { GenerationRequest, GenerationResult, ApiResponse } from '../api/client';

export type GenerationType = 'narrative' | 'story-element' | 'event';

export interface GenerationState {
  isGenerating: boolean;
  progress: number;
  result: GenerationResult | null;
  error: string | null;
}

export const useGeneration = () => {
  const [generations, setGenerations] = useState<GenerationResult[]>([]);
  const [generationState, setGenerationState] = useState<GenerationState>({
    isGenerating: false, progress: 0, result: null, error: null
  });

  const generateContent = useCallback(async (request: GenerationRequest) => {
    setGenerationState(prev => ({ ...prev, isGenerating: true, error: null, progress: 0 }));
    try {
      const response = await apiClient.generateContent(request);
      if (response.status === 'success' && response.data) {
        setGenerations(prev => [response.data, ...prev]);
        setGenerationState({ isGenerating: false, progress: 100, result: response.data, error: null });
        return response.data;
      } else {
        setGenerationState({ isGenerating: false, progress: 0, result: null, error: response.error?.message || 'Generation failed' });
        return null;
      }
    } catch (err) {
      setGenerationState({ isGenerating: false, progress: 0, result: null, error: err instanceof Error ? err.message : 'Unknown error' });
      return null;
    }
  }, []);

  const checkGenerationStatus = useCallback(async (generationId: string) => {
    try {
      const response = await apiClient.getGenerationStatus(generationId);
      if (response.status === 'success' && response.data) {
        setGenerations(prev => prev.map(g => g.id === generationId ? response.data! : g));
        return response.data;
      }
      return null;
    } catch (err) { console.error('Failed to check generation status:', err); return null; }
  }, []);

  const clearGenerations = useCallback(() => { setGenerations([]); }, []);
  return { generations, generationState, generateContent, checkGenerationStatus, clearGenerations };
};

export const useProjectGeneration = (projectId: string) => {
  const { generateContent, ...rest } = useGeneration();
  const generateNarrative = useCallback(async (prompt: string, parameters: Record<string, unknown> = {}) => {
    return generateContent({ project_id: projectId, element_type: 'narrative', prompt, parameters });
  }, [projectId, generateContent]);
  const generateStoryElement = useCallback(async (prompt: string, elementType: 'character' | 'event' | 'location' | 'object', narrativeId?: string, parameters: Record<string, unknown> = {}) => {
    return generateContent({
      project_id: projectId,
      narrative_id: narrativeId,
      element_type: 'story-element',
      prompt: `${prompt} (Type: ${elementType})`,
      parameters: { element_type: elementType, ...parameters }
    });
  }, [projectId, generateContent]);
  const generateEvent = useCallback(async (prompt: string, narrativeId?: string, parameters: Record<string, unknown> = {}) => {
    return generateContent({ project_id: projectId, narrative_id: narrativeId, element_type: 'event', prompt, parameters });
  }, [projectId, generateContent]);
  return { generateNarrative, generateStoryElement, generateEvent, ...rest };
};