import React, { useState } from 'react';
import { Button, Input } from '../Common';

interface GenerationFormProps {
  onGenerate: (type: 'narrative' | 'story-element' | 'event', prompt: string, parameters: Record<string, unknown>) => Promise<void>;
  isGenerating: boolean;
}

export default function GenerationForm({ onGenerate, isGenerating }: GenerationFormProps) {
  const [formData, setFormData] = useState({
    type: 'narrative' as 'narrative' | 'story-element' | 'event',
    prompt: '',
    temperature: 0.7,
    maxTokens: 1000,
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onGenerate(formData.type, formData.prompt, {
      temperature: formData.temperature,
      max_tokens: formData.maxTokens,
    });
  };

  const handleChange = (field: string, value: string | number) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Generation Type</label>
        <select
          value={formData.type}
          onChange={(e) => handleChange('type', e.target.value as 'narrative' | 'story-element' | 'event')}
          className="w-full px-3 py-2 border rounded-md"
        >
          <option value="narrative">Narrative</option>
          <option value="story-element">Story Element</option>
          <option value="event">Event</option>
        </select>
      </div>

      <Input
        label="Prompt"
        value={formData.prompt}
        onChange={(e) => handleChange('prompt', e.target.value)}
        required
        placeholder="Describe what you want to generate..."
      />

      <div className="grid grid-cols-2 gap-4">
        <Input
          label="Temperature"
          type="number"
          value={formData.temperature}
          onChange={(e) => handleChange('temperature', parseFloat(e.target.value) || 0.7)}
          min={0}
          max={1}
          step={0.1}
        />
        <Input
          label="Max Tokens"
          type="number"
          value={formData.maxTokens}
          onChange={(e) => handleChange('maxTokens', parseInt(e.target.value) || 1000)}
          min={100}
          max={4000}
        />
      </div>

      <Button type="submit" disabled={isGenerating || !formData.prompt.trim()}>
        {isGenerating ? 'Generating...' : 'Generate'}
      </Button>
    </form>
  );
}