import React, { useState, useEffect } from 'react';

export default function NarrativeGenerator() {
  const [narratives, setNarratives] = useState([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState(null);
  const [formData, setFormData] = useState({
    title: '',
    description: '',
    context: '',
    style: 'narrative',
  });

  useEffect(() => {
    loadNarratives();
  }, []);

  const loadNarratives = async () => {
    try {
      setIsLoading(true);
      // TODO: Implement API call to fetch narratives
      // const response = await fetchNarratives();
      // setNarratives(response.data);
      setError(null);
    } catch (err) {
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  };

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      setIsLoading(true);
      // TODO: Implement API call to create narrative
      // await createNarrative(formData);
      // await loadNarratives();
      // Reset form
      setFormData({ title: '', description: '', context: '', style: 'narrative' });
      setError(null);
    } catch (err) {
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  };

  if (isLoading) {
    return (
      <div className="narrative-generator">
        <div className="loading">Loading narratives...</div>
      </div>
    );
  }

  return (
    <div className="narrative-generator">
      <div className="generator-header">
        <h2>Narrative Generator</h2>
        <p>Create and manage story narratives for your projects</p>
      </div>

      {error && (
        <div className="error-message">{error}</div>
      )}

      <div className="generator-layout">
        <div className="generator-form">
          <h3>Create New Narrative</h3>
          <form onSubmit={handleSubmit}>
            <div className="form-group">
              <label htmlFor="title">Title</label>
              <input
                type="text"
                id="title"
                name="title"
                value={formData.title}
                onChange={handleInputChange}
                placeholder="Narrative title"
                required
              />
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
              <label htmlFor="context">Context</label>
              <textarea
                id="context"
                name="context"
                value={formData.context}
                onChange={handleInputChange}
                placeholder="Detailed context for the narrative"
                rows={5}
              />
            </div>

            <div className="form-group">
              <label htmlFor="style">Style</label>
              <select
                id="style"
                name="style"
                value={formData.style}
                onChange={handleInputChange}
              >
                <option value="narrative">Narrative</option>
                <option value="dialogue">Dialogue</option>
                <option value="descriptive">Descriptive</option>
                <option value="technical">Technical</option>
              </select>
            </div>

            <div className="form-actions">
              <button type="submit" disabled={isLoading} className="btn btn-primary">
                {isLoading ? 'Generating...' : 'Generate Narrative'}
              </button>
              <button type="button" className="btn btn-secondary">
                Clear
              </button>
            </div>
          </form>
        </div>

        <div className="narrative-list">
          <h3>Existing Narratives</h3>
          {narratives.length === 0 ? (
            <div className="empty-state">No narratives found. Create your first narrative.</div>
          ) : (
            <ul className="narrative-items">
              {narratives.map(narrative => (
                <li key={narrative.id} className="narrative-item">
                  <div className="narrative-info">
                    <h4>{narrative.title}</h4>
                    <p>{narrative.description}</p>
                    <span className="narrative-style">{narrative.style}</span>
                  </div>
                  <div className="narrative-actions">
                    <button className="btn-icon">View</button>
                    <button className="btn-icon">Edit</button>
                    <button className="btn-icon btn-danger">Delete</button>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>
    </div>
  );
}
