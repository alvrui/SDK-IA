# Contrato de API Interna - SDK-IA

**Version**: 1.0.0  
**Fecha**: 7 de julio de 2026  
**Estado**: Validado

---

## Indice

1. [Resumen y Convenciones](overview.md) - Vision general, convenciones y arquitectura
2. [Servicio Secretario](secretario.md) - Endpoints del modulo Python (Puerto 8000)
3. [Servicio SDK](sdk.md) - Endpoints del modulo Rust (Puerto 8080)
4. [Esquemas de Datos](schemas.md) - Definicion de todos los esquemas JSON
5. [Ejemplos de Uso](examples.md) - Ejemplos practicos con curl

---

## Descripcion General

Este contrato define la **API interna** para la comunicacion entre los dos modulos principales del proyecto SDK-IA:

- **Secretario (Python)**: Gestion de agentes Mistral, conversaciones y logs
- **SDK (Rust)**: Gestion de proyectos, validacion y generacion de contenido narrativo

### Caracteristicas de la API Interna

- **Prefijo**: /internal/ - Diferencia la API interna de la externa futura
- **Versionado**: /api/v1/ - Permite compatibilidad hacia atras
- **Formato**: JSON - Todas las comunicaciones en formato JSON
- **Bidireccional**: Rust <-> Python - Ambos modulos pueden llamarse mutuamente
- **Autenticacion**: Ninguna (solo para uso interno en localhost)

### Puertos

| Servicio | Puerto | Base URL |
|----------|--------|----------|
| Secretario | 8000 | http://localhost:8000/api/v1 |
| SDK | 8080 | http://localhost:8080/api/v1 |

---

## Convenciones Generales

### Formato de Respuesta

Todas las respuestas siguen el mismo formato:

```json
{
  "status": "success|error",
  "data": { ... },
  "error": { ... },
  "meta": { ... }
}
```

### Codigos de Estado HTTP

| Codigo | Descripcion | Uso |
|--------|-------------|-----|
| 200 | OK | Exito en operaciones de lectura |
| 201 | Created | Exito en operaciones de creacion |
| 204 | No Content | Exito en operaciones de eliminacion |
| 400 | Bad Request | Error en datos de entrada |
| 404 | Not Found | Recurso no encontrado |
| 422 | Unprocessable Entity | Error de validacion |
| 500 | Internal Server Error | Error interno del servidor |

### Headers Comunes

```http
Content-Type: application/json
Accept: application/json
X-Request-ID: <uuid>
X-Timestamp: <ISO-8601>
```

---

## Resumen de Endpoints

### Secretario (Python) - Puerto 8000

| Metodo | Endpoint | Descripcion |
|--------|----------|-------------|
| GET | /internal/agents | Listar agentes |
| POST | /internal/agents | Crear agente |
| GET | /internal/agents/{name} | Obtener agente |
| DELETE | /internal/agents/{name} | Eliminar agente |
| POST | /internal/agents/{name}/message | Enviar mensaje |
| GET | /internal/agents/{name}/conversation | Obtener conversacion |
| POST | /internal/agents/{name}/conversation/reset | Reiniciar conversacion |
| GET | /internal/logs | Listar logs |
| GET | /internal/health | Estado de salud |

### SDK (Rust) - Puerto 8080

| Metodo | Endpoint | Descripcion |
|--------|----------|-------------|
| GET | /internal/projects | Listar proyectos |
| POST | /internal/projects | Crear proyecto |
| GET | /internal/projects/{id} | Obtener proyecto |
| PUT | /internal/projects/{id} | Actualizar proyecto |
| DELETE | /internal/projects/{id} | Eliminar proyecto |
| POST | /internal/projects/{id}/validate | Validar proyecto |
| GET | /internal/catalog/{type} | Obtener catalogo |
| POST | /internal/generate | Generar contenido |
| GET | /internal/health | Estado de salud |

---

## Flujo de Trabajo Tipico

1. SDK recibe solicitud de generacion de contenido
2. SDK valida el contexto del proyecto
3. SDK envia mensaje a Secretario (POST /internal/agents/{name}/message)
4. Secretario envia mensaje a Mistral
5. Secretario recibe y normaliza respuesta
6. SDK procesa la respuesta y valida contra esquemas
7. SDK guarda el resultado en el proyecto
8. SDK devuelve resultado al cliente

---

## Archivos Relacionados

- [api-contract.yaml](api-contract.yaml) - Especificacion OpenAPI 3.0 (formato maquina)
- [PLAN.md](../PLAN.md) - Plan de ejecucion del proyecto
- [README.md](../README.md) - Documentacion principal del proyecto

---

## Historial de Cambios

| Version | Fecha | Cambios |
|--------|-------|---------|
| 1.0.0 | 2026-07-07 | Version inicial |
