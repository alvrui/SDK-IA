import React, { useState, useEffect } from 'react';
import { listProjects, Project } from '../../api/projects';
import { listNarratives, createNarrative, getHollywoodElements, checkCompatibility } from '../../api/narratives';
import LoadingSpinner from '../Common/LoadingSpinner';
import ErrorDisplay from '../Common/ErrorDisplay';
import './NarrativeGenerator.css';

interface StoryElement {
  id: string;
  name: string;
  type: string;
  description: string;
}

interface SelectedElement {
  element: StoryElement;
  position: number;
}

export default function NarrativeGenerator() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [selectedProject, setSelectedProject] = useState<string>('');
  const [narratives, setNarratives] = useState<any[]>([]);
  const [hollywoodElements, setHollywoodElements] = useState<StoryElement[]>([]);
  const [selectedElements, setSelectedElements] = useState<SelectedElement[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [narrativeName, setNarrativeName] = useState('');
  const [narrativeDescription, setNarrativeDescription] = useState('');
  const [compatibilityMatrix, setCompatibilityMatrix] = useState<Record<string, Record<string, number>>>(() => ({}));
  const [showCompatibility, setShowCompatibility] = useState(false);
  const [compatibilityResult, setCompatibilityResult] = useState<{element1: string; element2: string; score: number} | null>(null);

  useEffect(() => {
    const loadData = async () => {
      try {
        setLoading(true);
        const [projectsResponse, narrativesResponse, elementsResponse] = await Promise.all([
          listProjects().catch(() => ({ projects: [] })),
          listNarratives().catch(() => ({ narratives: [] })),
          getHollywoodElements().catch(() => [])
        ]);

        setProjects(projectsResponse.projects || []);
        setNarratives(narrativesResponse.narratives || []);
        setHollywoodElements(elementsResponse || []);

        // Cargar matriz de compatibilidad
        if (elementsResponse && elementsResponse.length > 0) {
          const matrix: Record<string, Record<string, number>> = {};
          for (const element of elementsResponse) {
            matrix[element.id] = {};
          }
          setCompatibilityMatrix(matrix);
        }

        setError(null);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load data');
      } finally {
        setLoading(false);
      }
    };

    loadData();
  }, []);

  const handleAddElement = (element: StoryElement) => {
    // Verificar compatibilidad con el último elemento añadido
    if (selectedElements.length > 0) {
      const lastElement = selectedElements[selectedElements.length - 1].element;
      checkCompatibility(lastElement.id, element.id)
        .then(result => {
          if (result.compatibility_score < 0.5) {
            if (!window.confirm(`La compatibilidad entre "${lastElement.name}" y "${element.name}" es baja (${result.compatibility_score}). ¿Deseas continuars?`)) {
              return;
            }
          }
          setSelectedElements([...selectedElements, { element, position: selectedElements.length }]);
          setCompatibilityResult(result);
        })
        .catch(() => {
          // Si falla la verificación, añadir de todos modos
          setSelectedElements([...selectedElements, { element, position: selectedElements.length }]);
        });
    } else {
      setSelectedElements([{ element, position: 0 }]);
    }
  };

  const handleRemoveElement = (position: number) => {
    setSelectedElements(selectedElements.filter(e => e.position !== position));
  };

  const handleMoveElement = (position: number, direction: 'up' | 'down') => {
    const newPosition = direction === 'up' ? position - 1 : position + 1;
    if (newPosition < 0 || newPosition >= selectedElements.length) return;

    const updatedElements = [...selectedElements];
    const elementToMove = updatedElements.find(e => e.position === position);
    const elementToSwap = updatedElements.find(e => e.position === newPosition);

    if (elementToMove && elementToSwap) {
      elementToMove.position = newPosition;
      elementToSwap.position = position;
      updatedElements.sort((a, b) => a.position - b.position);
      setSelectedElements(updatedElements);
    }
  };

  const handleGenerateNarrative = async () => {
    if (!selectedProject) {
      setError('Please select a project');
      return;
    }

    if (selectedElements.length === 0) {
      setError('Please add at least one story element');
      return;
    }

    if (!narrativeName) {
      setError('Please enter a narrative name');
      return;
    }

    try {
      const narrativeData = {
        name: narrativeName,
        description: narrativeDescription,
        project_id: selectedProject,
        story_elements: selectedElements.map(e => ({
          element_id: e.element.id,
          position: e.position,
          type: e.element.type
        }))
      };

      const newNarrative = await createNarrative(narrativeData as any);
      setNarratives([...narratives, newNarrative]);
      setNarrativeName('');
      setNarrativeDescription('');
      setSelectedElements([]);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to generate narrative');
    }
  };

  const handleCheckCompatibility = async (element1Id: string, element2Id: string) => {
    try {
      const result = await checkCompatibility(element1Id, element2Id);
      setCompatibilityResult(result);
      setShowCompatibility(true);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to check compatibility');
    }
  };

  if (loading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorDisplay message={error} onRetry={() => window.location.reload()} />;
  }

  return (
    <div className="narrative-generator">
      <div className="generator-header">
        <h2>Generador de Narrativas Cadiz12</h2>
        <p>Crea narrativas utilizando elementos de Hollywood Animal con validación de compatibilidad</p>
      </div>

      <div className="generator-layout">
        <div className="elements-panel">
          <div className="panel-header">
            <h3>Elementos Hollywood Animal</h3>
            <div className="element-count">{hollywoodElements.length} elementos disponibles</div>
          </div>
          <div className="elements-list">
            {hollywoodElements.length === 0 ? (
              <div className="empty-state">No hay elementos de Hollywood Animal cargados</div>
            ) : (
              hollywoodElements.map(element => (
                <div
                  key={element.id}
                  className="element-card"
                  onClick={() => handleAddElement(element)}
                >
                  <div className="element-info">
                    <strong>{element.name}</strong>
                    <span className="element-type">{element.type}</span>
                  </div>
                  <p className="element-description">{element.description}</p>
                </div>
              ))
            )}
          </div>
        </div>

        <div className="narrative-panel">
          <div className="panel-header">
            <h3>Narrativa en Construcción</h3>
            <div className="narrative-actions">
              <button
                className="button"
                onClick={() => setSelectedElements([])}
                disabled={selectedElements.length === 0}
              >
                Limpiar
              </button>
            </div>
          </div>

          <div className="narrative-builder">
            <div className="form-group">
              <label>Nombre de la Narrativa</label>
              <input
                type="text"
                value={narrativeName}
                onChange={(e) => setNarrativeName(e.target.value)}
                placeholder="Ej: La Gran Aventura"
              />
            </div>

            <div className="form-group">
              <label>Descripción</label>
              <textarea
                value={narrativeDescription}
                onChange={(e) => setNarrativeDescription(e.target.value)}
                placeholder="Descripción de la narrativa..."
                rows={3}
              />
            </div>

            <div className="form-group">
              <label>Proyecto</label>
              <select
                value={selectedProject}
                onChange={(e) => setSelectedProject(e.target.value)}
              >
                <option value="">Selecciona un proyecto</option>
                {projects.map(project => (
                  <option key={project.id} value={project.id}>
                    {project.name}
                  </option>
                ))}
              </select>
            </div>

            <div className="selected-elements">
              <h4>Elementos Seleccionados ({selectedElements.length})</h4>
              {selectedElements.length === 0 ? (
                <div className="empty-state">
                  <p>Añade elementos para empezar a construir tu narrativa</p>
                  <small>Los elementos se validarán automáticamente por compatibilidad</small>
                </div>
              ) : (
                <div className="elements-sequence">
                  {selectedElements.map((selected, index) => (
                    <div key={`${selected.element.id}-${index}`} className="selected-element">
                      <div className="element-header">
                        <strong>{selected.element.name}</strong>
                        <span className="element-type">{selected.element.type}</span>
                      </div>
                      <div className="element-actions">
                        <button
                          className="button-icon"
                          onClick={() => handleMoveElement(selected.position, 'up')}
                          disabled={selected.position === 0}
                          title="Mover arriba"
                        >
                          ↑
                        </button>
                        <button
                          className="button-icon"
                          onClick={() => handleMoveElement(selected.position, 'down')}
                          disabled={selected.position === selectedElements.length - 1}
                          title="Mover abajo"
                        >
                          ↓
                        </button>
                        <button
                          className="button-icon danger"
                          onClick={() => handleRemoveElement(selected.position)}
                          title="Eliminar"
                        >
                          ×
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>

            {compatibilityResult && showCompatibility && (
              <div className="compatibility-result">
                <h4>Resultado de Compatibilidad</h4>
                <p>
                  <strong>{compatibilityResult.element1}</strong> y
                  <strong>{compatibilityResult.element2}</strong>:
                </p>
                <p>
                  Puntuación: <span className={`score ${compatibilityResult.score >= 0.7 ? 'high' : compatibilityResult.score >= 0.4 ? 'medium' : 'low'}`}>
                    {compatibilityResult.score.toFixed(2)}
                  </span>
                </p>
              </div>
            )}

            <div className="generate-actions">
              <button
                className="button primary"
                onClick={handleGenerateNarrative}
                disabled={!narrativeName || !selectedProject || selectedElements.length === 0}
              >
                Generar Narrativa
              </button>
            </div>
          </div>
        </div>
      </div>

      <div className="existing-narratives">
        <h3>Narrativas Existentes</h3>
        {narratives.length === 0 ? (
          <div className="empty-state">No hay narrativas generadas aún</div>
        ) : (
          <div className="narratives-list">
            {narratives.map(narrative => (
              <div key={narrative.id} className="narrative-card">
                <div className="narrative-header">
                  <h4>{narrative.name}</h4>
                  <span className="narrative-project">
                    Proyecto: {projects.find(p => p.id === narrative.project_id)?.name || narrative.project_id}
                  </span>
                </div>
                <p>{narrative.description}</p>
                <div className="narrative-meta">
                  <span>{narrative.story_elements?.length || 0} elementos</span>
                  <span>Creado: {new Date(narrative.created_at).toLocaleDateString()}</span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
