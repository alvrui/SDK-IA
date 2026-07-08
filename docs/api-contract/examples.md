# Ejemplos de Uso - API Interna

## Ejemplo 1: Crear Agente y Enviar Mensaje

### Crear Agente
POST /api/v1/internal/agents
{
  name: CoordinadorNarrativo
  description: Coordinador de narrativas
  agent_id: ag_019f34c158827241b3bfe6d6a396cd89
}

### Enviar Mensaje
POST /api/v1/internal/agents/CoordinadorNarrativo/message
{
  message: Genera una narrativa para el acto 1
  options: {temperature: 0.7, max_tokens: 2048}
}

## Ejemplo 2: Crear Proyecto y Validar

### Crear Proyecto
POST /api/v1/internal/projects
{
  title: Campana de Cadiz 1812
  act: act_1
  tone: serious
  settings: {agents: {narrative: CoordinadorNarrativo}}
}

### Validar Proyecto
POST /api/v1/internal/projects/proj_001/validate

## Ejemplo 3: Generar Contenido

POST /api/v1/internal/generate
{
  project_id: proj_001
  section: narrative
  action: generate
  agent_name: CoordinadorNarrativo
  context: {act: act_1, tone: serious}
}

## Ejemplo 4: Flujo Completo

1. SDK valida contexto
2. SDK -> Secretario: POST /internal/agents/{name}/message
3. Secretario -> Mistral: enviar mensaje
4. Secretario -> SDK: respuesta normalizada
5. SDK valida y guarda en proyecto
6. SDK -> Cliente: resultado final

## Ejemplo 5: Obtener Catalogo

GET /api/v1/internal/catalog/factions

## Ejemplo 6: Salud

GET /api/v1/internal/health (para ambos servicios)

## Codigos de Error

400 Bad Request: Datos de entrada invalidos
404 Not Found: Recurso no encontrado
422 Unprocessable Entity: Error de validacion
500 Internal Server Error: Error interno