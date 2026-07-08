# Servicio SDK - API Interna

Base URL: http://localhost:8080/api/v1
Puerto: 8080
Modulo: Rust

## Endpoints

### Proyectos
- GET /internal/projects - Listar todos los proyectos
- POST /internal/projects - Crear nuevo proyecto
- GET /internal/projects/{id} - Obtener proyecto especifico
- PUT /internal/projects/{id} - Actualizar proyecto
- DELETE /internal/projects/{id} - Eliminar proyecto
- POST /internal/projects/{id}/validate - Validar proyecto

### Catalogos
- GET /internal/catalog/{type} - Obtener catalogo (factions, characters, scenarios, themes, stakes, tags)

### Generacion
- POST /internal/generate - Generar contenido con IA

### Salud
- GET /internal/health - Estado de salud

## Esquemas

Project: {id, title, summary, act, tone, historical_scope, settings, metadata}
GenerationRequest: {project_id, section, action, agent_name, context, options}
GenerationResponse: {section, action, agent_name, generated_at, result, conversation_id, duration_ms}