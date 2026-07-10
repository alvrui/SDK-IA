import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { listProjects, deleteProject, createProject, updateProject, Project } from '../../api/projects';
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
      setProjects(response.projects);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load projects');
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: string) => {
    if (!window.confirm('Are you sure you want to delete this project?')) return;
    try {
      await deleteProject(id);
      setProjects(projects.filter(p => p.id !== id));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete project');
    }
  };

  const handleCreate = async (project: { name: string; description: string }) => {
    try {
      const newProject = await createProject(project);
      setProjects([...projects, newProject]);
      setShowForm(false);
      setEditingProject(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create project');
    }
  };

  const handleUpdate = async (id: string, project: { name: string; description: string }) => {
    try {
      const updatedProject = await updateProject(id, project);
      setProjects(projects.map(p => p.id === id ? updatedProject : p));
      setShowForm(false);
      setEditingProject(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update project');
    }
  };

  if (loading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorDisplay message={error} onRetry={loadProjects} />;
  }

  return (
    <div className="project-list">
      <div className="project-header">
        <h2>Proyectos Cadiz12</h2>
        <button
          className="button primary"
          onClick={() => {
            setShowForm(true);
            setEditingProject(null);
          }}
        >
          + Nuevo Proyecto
        </button>
      </div>

      {showForm && (
        <ProjectForm
          project={editingProject}
          onSubmit={editingProject ? handleUpdate : handleCreate}
          onCancel={() => {
            setShowForm(false);
            setEditingProject(null);
          }}
        />
      )}

      {projects.length === 0 ? (
        <div className="empty-state">
          <p>No hay proyectos creados aún.</p>
          <button className="button" onClick={() => setShowForm(true)}>
            Crear primer proyecto
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
