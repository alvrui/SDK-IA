# Servicio Secretario - API Interna

Base URL: http://localhost:8000/api/v1
Puerto: 8000
Modulo: Python

## Endpoints

### Agentes
- GET /internal/agents - Listar todos los agentes
- POST /internal/agents - Crear nuevo agente
- GET /internal/agents/{name} - Obtener agente especifico
- DELETE /internal/agents/{name} - Eliminar agente

### Mensajes
- POST /internal/agents/{name}/message - Enviar mensaje a agente
- GET /internal/agents/{name}/conversation - Obtener conversacion
- POST /internal/agents/{name}/conversation/reset - Reiniciar conversacion

### Logs
- GET /internal/logs - Listar logs (filtros: agent_name, limit, since, until)

### Salud
- GET /internal/health - Estado de salud

## Esquemas

Agent: {name, id, description, status, conversation_id, last_used_at, created_at}
MessageRequest: {message, conversation_id, force_new, options}
MessageResponse: {conversation_id, agent_name, message, response, timestamp, duration_ms}