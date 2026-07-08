# Contrato de API Interna - SDK-IA - Resumen

Version 1.0.0 - 7 julio 2026
Ambito: API interna entre modulos Rust (SDK) y Python (Secretario)

## Introduccion

Este documento define los contratos de API interna para la comunicacion entre los dos modulos principales:
- Secretario (Python): Gestion de agentes Mistral, conversaciones y logs
- SDK (Rust): Gestion de proyectos, validacion y generacion de contenido narrativo

La API interna utiliza el prefijo /internal/ para diferenciarla de la API externa futura.

## Arquitectura

SDK (Rust) Puerto 8080 <-> Secretario (Python) Puerto 8000

## Convenciones Generales

### Formato de Respuesta
Todas las respuestas siguen el formato:
json
{
  status: success|error
  data: { ... }
  error: { ... }
  meta: { ... }
}

### Codigos de Estado HTTP
200 OK, 201 Created, 204 No Content, 400 Bad Request, 404 Not Found, 422 Unprocessable Entity, 500 Internal Server Error

### Headers Comunes
Content-Type: application/json
Accept: application/json
X-Request-ID: uuid
X-Timestamp: ISO-8601

## Servicios y Endpoints

### Secretario (Python) Puerto 8000
GET /internal/agents - Listar agentes
POST /internal/agents - Crear agente
GET /internal/agents/{name} - Obtener agente
DELETE /internal/agents/{name} - Eliminar agente
POST /internal/agents/{name}/message - Enviar mensaje
GET /internal/agents/{name}/conversation - Obtener conversacion
POST /internal/agents/{name}/conversation/reset - Reiniciar conversacion
GET /internal/logs - Listar logs
GET /internal/health - Estado de salud

### SDK (Rust) Puerto 8080
GET /internal/projects - Listar proyectos
POST /internal/projects - Crear proyecto
GET /internal/projects/{id} - Obtener proyecto
PUT /internal/projects/{id} - Actualizar proyecto
DELETE /internal/projects/{id} - Eliminar proyecto
POST /internal/projects/{id}/validate - Validar proyecto
GET /internal/catalog/{type} - Obtener catalogo
POST /internal/generate - Generar contenido
GET /internal/health - Estado de salud