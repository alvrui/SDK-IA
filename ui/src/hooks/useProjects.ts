import { useState, useEffect, useCallback } from 'react';
import apiClient, { Project, ApiResponse } from '../api/client';

export const useProjects = () => {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchProjects = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.getProjects();
      if (response.status === 'success') setProjects(response.data || []);
      else setError(response.error?.message || 'Failed to fetch projects');
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); }
    finally { setLoading(false); }
  }, []);

  const createProject = useCallback(async (projectData: Omit<Project, 'id' | 'created_at' | 'updated_at'>) => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.createProject(projectData);
      if (response.status === 'success') { setProjects(prev => [response.data!, ...prev]); return response.data; }
      else { setError(response.error?.message || 'Failed to create project'); return null; }
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); return null; }
    finally { setLoading(false); }
  }, []);

  const updateProject = useCallback(async (id: string, projectData: Partial<Project>) => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.updateProject(id, projectData);
      if (response.status === 'success') { setProjects(prev => prev.map(p => p.id === id ? response.data! : p)); return response.data; }
      else { setError(response.error?.message || 'Failed to update project'); return null; }
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); return null; }
    finally { setLoading(false); }
  }, []);

  const deleteProject = useCallback(async (id: string) => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.deleteProject(id);
      if (response.status === 'success') { setProjects(prev => prev.filter(p => p.id !== id)); return true; }
      else { setError(response.error?.message || 'Failed to delete project'); return false; }
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); return false; }
    finally { setLoading(false); }
  }, []);

  useEffect(() => { fetchProjects(); }, [fetchProjects]);
  return { projects, loading, error, fetchProjects, createProject, updateProject, deleteProject };
};

export const useProject = (id: string) => {
  const [project, setProject] = useState<Project | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchProject = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await apiClient.getProject(id);
      if (response.status === 'success') setProject(response.data || null);
      else setError(response.error?.message || 'Failed to fetch project');
    } catch (err) { setError(err instanceof Error ? err.message : 'Unknown error'); }
    finally { setLoading(false); }
  }, [id]);

  useEffect(() => { if (id) fetchProject(); }, [id, fetchProject]);
  return { project, loading, error, fetchProject };
};