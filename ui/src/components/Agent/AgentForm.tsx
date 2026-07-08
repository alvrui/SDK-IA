import React, { useState, useEffect } from 'react';
import { Agent } from '../../types';
import { Input, Button } from '../Common';

interface AgentFormProps {
  agent?: Agent | null;
  onSubmit: (data: Omit<Agent, 'id' | 'created_at'>) => Promise<void>;
  onClose: () => void;
}

export default function AgentForm({ agent, onSubmit, onClose }: AgentFormProps) {
  const [formData, setFormData] = useState({
    name: '',
    model: 'mistral-tiny',
    description: '',
  });
  const [errors, setErrors] = useState<Record<string, string>>({});

  useEffect(() => {
    if (agent) {
      setFormData({
        name: agent.name,
        model: agent.model,
        description: agent.description,
      });
    }
  }, [agent]);

  const validate = () => {
    const newErrors: Record<string, string> = {};
    if (!formData.name.trim()) newErrors.name = 'Name is required';
    if (!formData.model.trim()) newErrors.model = 'Model is required';
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validate()) return;
    await onSubmit(formData);
  };

  const handleChange = (field: string, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    if (errors[field]) setErrors(prev => ({ ...prev, [field]: '' }));
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <Input
        label="Agent Name"
        value={formData.name}
        onChange={(e) => handleChange('name', e.target.value)}
        error={errors.name}
        required
      />
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Model</label>
        <select
          value={formData.model}
          onChange={(e) => handleChange('model', e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          <option value="mistral-tiny">Mistral Tiny</option>
          <option value="mistral-small">Mistral Small</option>
          <option value="mistral-medium">Mistral Medium</option>
          <option value="mistral-large">Mistral Large</option>
        </select>
      </div>
      <Input
        label="Description"
        value={formData.description}
        onChange={(e) => handleChange('description', e.target.value)}
        error={errors.description}
      />
      <div className="flex justify-end space-x-3 pt-4">
        <Button variant="secondary" onClick={onClose}>
          Cancel
        </Button>
        <Button type="submit" variant="primary">
          {agent ? 'Update' : 'Create'}
        </Button>
      </div>
    </form>
  );
}