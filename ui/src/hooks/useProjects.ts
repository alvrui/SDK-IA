import { useState, useEffect, useCallback } from 'react';
import { listProjects, createProject, deleteProject, Project, ProjectCreate } from '../api/projects';

export function useProjects() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchProjects = useCallback(async () => {
    try {
      setLoading(true);
      console.log('[useProjects] Fetching projects...');
      const response = await listProjects();
      console.log('[useProjects] Response:', response);
      setProjects(response.projects || []);
      setError(null);
    } catch (err) {
      console.error('[useProjects] Error:', err);
      setError(err instanceof Error ? err.message : 'Failed to load projects');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchProjects();
  }, [fetchProjects]);

  const createProjectWrapper = async (project: ProjectCreate) => {
    try {
      console.log('[useProjects] Creating project:', project);
      const newProject = await createProject(project);
      console.log('[useProjects] Created:', newProject);
      setProjects([...projects, newProject]);
      return newProject;
    } catch (err) {
      console.error('[useProjects] Create error:', err);
      setError(err instanceof Error ? err.message : 'Failed to create project');
      throw err;
    }
  };

  const deleteProjectWrapper = async (id: string) => {
    try {
      console.log('[useProjects] Deleting project:', id);
      await deleteProject(id);
      setProjects(projects.filter(p => p.id !== id));
    } catch (err) {
      console.error('[useProjects] Delete error:', err);
      setError(err instanceof Error ? err.message : 'Failed to delete project');
      throw err;
    }
  };

  return {
    projects,
    loading,
    error,
    fetchProjects,
    createProject: createProjectWrapper,
    deleteProject: deleteProjectWrapper
  };
}