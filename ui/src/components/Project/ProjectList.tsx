import React, { useState } from 'react';
import { useProjects } from '../../hooks';
import { Button, LoadingSpinner, ErrorDisplay, Modal } from '../Common';
import ProjectForm from './ProjectForm';
import ProjectCard from './ProjectCard';
import { Project } from '../../api/client';

export default function ProjectList() {
  const { projects, loading, error, fetchProjects, createProject, updateProject, deleteProject } = useProjects();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedProject, setSelectedProject] = useState<Project | null>(null);

  const handleCreate = async (data) => {
    await createProject(data);
    await fetchProjects();
    setIsModalOpen(false);
  };

  const handleUpdate = async (id: string, data) => {
    await updateProject(id, data);
    await fetchProjects();
    setIsModalOpen(false);
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this project?')) {
      await deleteProject(id);
      await fetchProjects();
    }
  };

  const handleEdit = (project: Project) => {
    setSelectedProject(project);
    setIsModalOpen(true);
  };

  const handleCloseModal = () => {
    setIsModalOpen(false);
    setSelectedProject(null);
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold">Projects</h1>
        <Button onClick={() => { setSelectedProject(null); setIsModalOpen(true); }}>
          Create Project
        </Button>
      </div>

      {loading && <LoadingSpinner text="Loading projects..." />}
      {error && <ErrorDisplay error={error} onRetry={fetchProjects} />}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {projects.map(project => (
          <ProjectCard
            key={project.id}
            project={project}
            onEdit={() => handleEdit(project)}
            onDelete={() => handleDelete(project.id)}
          />
        ))}
      </div>

      <Modal isOpen={isModalOpen} onClose={handleCloseModal} title={selectedProject ? 'Edit Project' : 'Create Project'}>
        <ProjectForm 
          project={selectedProject} 
          onSubmit={selectedProject ? (data) => handleUpdate(selectedProject.id, data) : handleCreate}
          onClose={handleCloseModal} 
        />
      </Modal>
    </div>
  );
}