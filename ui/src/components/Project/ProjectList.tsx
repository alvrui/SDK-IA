import React, { useState, useEffect } from 'react';
import { listProjects, createProject, deleteProject, Project } from '../../api/projects';
import ProjectCard from './ProjectCard';
import ProjectForm from './ProjectForm';
import LoadingSpinner from '../Common/LoadingSpinner';
import ErrorDisplay from '../Common/ErrorDisplay';
import './ProjectList.css';

export default function ProjectList() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [editingProject, setEditingProject] = useState<Project | null>(null);

  useEffect(() => {
    loadProjects();
  }, []);

  const loadProjects = async () => {
    try {
      setLoading(true);
      const response = await listProjects();
      setProjects(response.projects || []);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load projects');
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = async (project: { name: string; description: string }) => {
    try {
      const newProject = await createProject(project);
      setProjects([...projects, newProject]);
      setShowForm(false);
      setEditingProject(null);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create project');
    }
  };

  const handleUpdate = async (id: string, project: { name: string; description: string }) => {
    try {
      // Note: updateProject might not exist yet, using create as fallback
      const updatedProject = await createProject({...project, id});
      setProjects(projects.map(p => p.id === id ? updatedProject : p));
      setShowForm(false);
      setEditingProject(null);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update project');
    }
  };

  const handleDelete = async (id: string) => {
    if (!window.confirm('Are you sure you want to delete this project?')) return;
    try {
      await deleteProject(id);
      setProjects(projects.filter(p => p.id !== id));
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete project');
    }
  };

  if (loading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorDisplay message={error} onRetry={loadProjects} />;
  }

  return (
    <div className="project-list-container">
      <div className="project-header">
        <h2>Cadiz12 Projects</h2>
        <button
          className="btn btn-primary"
          onClick={() => {
            setShowForm(true);
            setEditingProject(null);
          }}
        >
          + Create Project
        </button>
      </div>

      {showForm && (
        <div className="project-form-modal">
          <ProjectForm
            project={editingProject}
            onSubmit={editingProject ? handleUpdate : handleCreate}
            onCancel={() => {
              setShowForm(false);
              setEditingProject(null);
            }}
          />
        </div>
      )}

      {projects.length === 0 ? (
        <div className="empty-state">
          <p>No projects found. Create your first Cadiz12 project.</p>
          <button
            className="btn btn-secondary"
            onClick={() => {
              setShowForm(true);
              setEditingProject(null);
            }}
          >
            Create Project
          </button>
        </div>
      ) : (
        <div className="projects-grid">
          {projects.map(project => (
            <ProjectCard
              key={project.id}
              project={project}
              onEdit={() => {
                setEditingProject(project);
                setShowForm(true);
              }}
              onDelete={() => handleDelete(project.id)}
            />
          ))}
        </div>
      )}
    </div>
  );
}
