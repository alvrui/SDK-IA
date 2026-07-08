import React, { useState } from 'react';
import { useProjects, useProjectGeneration } from '../../hooks';
import { LoadingSpinner, ErrorDisplay } from '../Common';
import GenerationForm from './GenerationForm';
import GenerationResults from './GenerationResults';

export default function GenerationInterface() {
  const { projects, loading: projectsLoading, error: projectsError } = useProjects();
  const [selectedProjectId, setSelectedProjectId] = useState<string>('');
  const { generateNarrative, generateStoryElement, generateEvent, generations, generationState } = useProjectGeneration(selectedProjectId);

  const handleGenerate = async (type: 'narrative' | 'story-element' | 'event', prompt: string, parameters: Record<string, unknown> = {}) => {
    switch (type) {
      case 'narrative':
        await generateNarrative(prompt, parameters);
        break;
      case 'story-element':
        await generateStoryElement(prompt, 'character', undefined, parameters);
        break;
      case 'event':
        await generateEvent(prompt, undefined, parameters);
        break;
    }
  };

  if (projectsLoading) return <LoadingSpinner text="Loading projects..." />;
  if (projectsError) return <ErrorDisplay error={projectsError} />;

  return (
    <div className="space-y-6">
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Select Project</label>
        <select
          value={selectedProjectId}
          onChange={(e) => setSelectedProjectId(e.target.value)}
          className="px-3 py-2 border rounded-md w-full max-w-md"
        >
          <option value="">Select a project</option>
          {projects.map(project => (
            <option key={project.id} value={project.id}>{project.name}</option>
          ))}
        </select>
      </div>

      {selectedProjectId && (
        <>
          <GenerationForm onGenerate={handleGenerate} isGenerating={generationState.isGenerating} />
          <GenerationResults generations={generations} />
        </>
      )}

      {generationState.error && <ErrorDisplay error={generationState.error} />}
    </div>
  );
}