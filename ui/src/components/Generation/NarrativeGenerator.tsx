import React, { useState, useEffect } from 'react';
import { listNarratives } from '../../api/narratives';
import { listProjects } from '../../api/projects';
import LoadingSpinner from '../Common/LoadingSpinner';
import ErrorDisplay from '../Common/ErrorDisplay';
import './NarrativeGenerator.css';

interface Narrative {
  id: string;
  title: string;
  description: string;
  context?: string;
  style: string;
  created_at?: string;
}

interface Project {
  id: string;
  name: string;
}

export default function NarrativeGenerator() {
  const [narratives, setNarratives] = useState<Narrative[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [formData, setFormData] = useState({
    title: '',
    description: '',
    context: '',
    style: 'narrative',
    project_id: ''
  });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const [narrativesResponse, projectsResponse] = await Promise.all([
        listNarratives().catch(() => ({ narratives: [] })),
        listProjects().catch(() => ({ projects: [] }))
      ]);
      setNarratives(narrativesResponse.narratives || []);
      setProjects(projectsResponse.projects || []);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load data');
    } finally {
      setLoading(false);
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setLoading(true);
      // TODO: Implement actual API call when backend is ready
      // For now, just add to local state
      const newNarrative: Narrative = {
        id: `narr-${Date.now()}`,
        title: formData.title,
        description: formData.description,
        context: formData.context,
        style: formData.style,
        created_at: new Date().toISOString()
      };
      setNarratives([...narratives, newNarrative]);
      setFormData({ title: '', description: '', context: '', style: 'narrative', project_id: '' });
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create narrative');
    } finally {
      setLoading(false);
    }
  };

  if (loading && narratives.length === 0) {
    return <LoadingSpinner />;
  }

  return (
    <div className="narrative-generator">
      <div className="generator-header">
        <h2>Narrative Generator</h2>
        <p>Create and manage story narratives for your Cadiz12 projects</p>
      </div>

      {error && (
        <div className="error-message">{error}</div>
      )}

      <div className="generator-content">
        <div className="generator-form-container">
          <h3>Create New Narrative</h3>
          <form onSubmit={handleSubmit} className="generator-form">
            <div className="form-group">
              <label htmlFor="title">Title</label>
              <input
                type="text"
                id="title"
                name="title"
                value={formData.title}
                onChange={handleInputChange}
                placeholder="Narrative title (e.g., The Great Adventure)"
                required
              />
            </div>

            <div className="form-group">
              <label htmlFor="project_id">Project</label>
              <select
                id="project_id"
                name="project_id"
                value={formData.project_id}
                onChange={handleInputChange}
                required
              >
                <option value="">Select a project</option>
                {projects.map(project => (
                  <option key={project.id} value={project.id}>
                    {project.name}
                  </option>
                ))}
              </select>
            </div>

            <div className="form-group">
              <label htmlFor="description">Description</label>
              <textarea
                id="description"
                name="description"
                value={formData.description}
                onChange={handleInputChange}
                placeholder="Brief description of the narrative"
                rows={3}
              />
            </div>

            <div className="form-group">
              <label htmlFor="context">Context / Background</label>
              <textarea
                id="context"
                name="context"
                value={formData.context}
                onChange={handleInputChange}
                placeholder="Detailed context, setting, or background for the narrative"
                rows={5}
              />
            </div>

            <div className="form-group">
              <label htmlFor="style">Narrative Style</label>
              <select
                id="style"
                name="style"
                value={formData.style}
                onChange={handleInputChange}
              >
                <option value="narrative">Standard Narrative</option>
                <option value="dialogue">Dialogue Format</option>
                <option value="descriptive">Descriptive</option>
                <option value="technical">Technical Documentation</option>
                <option value="story">Story Format</option>
              </select>
            </div>

            <div className="form-actions">
              <button
                type="submit"
                className="btn btn-primary"
                disabled={loading || !formData.title || !formData.project_id}
              >
                {loading ? 'Creating...' : 'Create Narrative'}
              </button>
              <button
                type="button"
                className="btn btn-secondary"
                onClick={() => setFormData({ title: '', description: '', context: '', style: 'narrative', project_id: '' })}
              >
                Clear Form
              </button>
            </div>
          </form>
        </div>

        <div className="narratives-section">
          <h3>Existing Narratives ({narratives.length})</h3>
          {narratives.length === 0 ? (
            <div className="empty-state">
              <p>No narratives found.</p>
              <p>Create your first narrative to get started.</p>
            </div>
          ) : (
            <div className="narratives-grid">
              {narratives.map(narrative => (
                <div key={narrative.id} className="narrative-card">
                  <div className="narrative-header">
                    <h4>{narrative.title}</h4>
                    <span className={`style-badge ${narrative.style}`}>
                      {narrative.style}
                    </span>
                  </div>
                  <p className="narrative-description">{narrative.description}</p>
                  {narrative.context && (
                    <p className="narrative-context">{narrative.context.substring(0, 100)}...</p>
                  )}
                  <div className="narrative-meta">
                    <span>Created: {new Date(narrative.created_at || '').toLocaleDateString()}</span>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
