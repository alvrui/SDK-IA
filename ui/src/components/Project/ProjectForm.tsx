import React, { useState, useEffect } from 'react';
import { Project } from '../../types';
import { Input, Button } from '../Common';

interface ProjectFormProps {
  project?: Project | null;
  onSubmit: (data: Omit<Project, 'id' | 'created_at' | 'updated_at' | 'version'>) => Promise<void>;
  onClose: () => void;
}

export default function ProjectForm({ project, onSubmit, onClose }: ProjectFormProps) {
  const [formData, setFormData] = useState({
    name: '',
    description: '',
    author: '',
    status: 'draft' as 'draft' | 'active' | 'archived',
    tags: [] as string[],
    metadata: {} as Record<string, string>,
  });
  const [errors, setErrors] = useState<Record<string, string>>({});

  useEffect(() => {
    if (project) {
      setFormData({
        name: project.name,
        description: project.description,
        author: project.author,
        status: project.status,
        tags: project.tags || [],
        metadata: project.metadata || {},
      });
    }
  }, [project]);

  const validate = () => {
    const newErrors: Record<string, string> = {};
    if (!formData.name.trim()) newErrors.name = 'Name is required';
    if (!formData.author.trim()) newErrors.author = 'Author is required';
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validate()) return;
    await onSubmit({
      ...formData,
      version: project?.version || '1.0.0',
    });
  };

  const handleChange = (field: string, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    if (errors[field]) setErrors(prev => ({ ...prev, [field]: '' }));
  };

  const handleTagsChange = (value: string) => {
    setFormData(prev => ({ ...prev, tags: value.split(',').map(t => t.trim()).filter(t => t) }));
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <Input
        label="Project Name"
        value={formData.name}
        onChange={(e) => handleChange('name', e.target.value)}
        error={errors.name}
        required
      />
      <Input
        label="Description"
        value={formData.description}
        onChange={(e) => handleChange('description', e.target.value)}
        error={errors.description}
      />
      <Input
        label="Author"
        value={formData.author}
        onChange={(e) => handleChange('author', e.target.value)}
        error={errors.author}
        required
      />
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Status</label>
        <select
          value={formData.status}
          onChange={(e) => handleChange('status', e.target.value as 'draft' | 'active' | 'archived')}
          className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="draft">Draft</option>
          <option value="active">Active</option>
          <option value="archived">Archived</option>
        </select>
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Tags (comma separated)</label>
        <Input
          value={formData.tags.join(', ')}
          onChange={(e) => handleTagsChange(e.target.value)}
          placeholder="e.g., adventure, fantasy"
        />
      </div>
      <div className="flex justify-end space-x-3 pt-4">
        <Button variant="secondary" onClick={onClose}>
          Cancel
        </Button>
        <Button type="submit" variant="primary">
          {project ? 'Update' : 'Create'}
        </Button>
      </div>
    </form>
  );
}