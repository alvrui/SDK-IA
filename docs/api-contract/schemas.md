# Esquemas de Datos - API Interna

## Esquemas Comunes

BaseResponse: {status: success|error, data: {}, error: {}, meta: {}}
ErrorResponse: {status: error, error: {code, message, details}}

## Agentes

Agent: {name, id, description, status, conversation_id, last_used_at, created_at, configuration}
AgentConfiguration: {model, temperature, max_tokens}
AgentCreateRequest: {name, description, agent_id}
MessageRequest: {message, conversation_id, force_new, options}
MessageResponse: {conversation_id, agent_name, message, response, timestamp, duration_ms}
Message: {role: user|agent|system, content, timestamp}

## Proyectos

Project: {id, title, summary, act, tone, historical_scope, spaces, factions, stakes, tags, story_elements, events, ai_history, settings, metadata}
ProjectSettings: {agents: {narrative, story_elements, events, validation, export}}
ProjectCreateRequest: {title, summary, act, tone, historical_scope, settings}
ProjectUpdateRequest: {title, summary, story_elements, events}

## Validacion

ValidationResponse: {valid, errors, warnings, checked_at}
ValidationIssue: {field, message, severity}

## Catalogos

CatalogItem: {id, name, description, tags}

## Generacion

GenerationRequest: {project_id, section, action, agent_name, context, options}
GenerationResponse: {section, action, agent_name, generated_at, result, conversation_id, duration_ms}
GenerationResult: {status, data, warnings}

## Logs

LogEntry: {id, agent_name, agent_id, conversation_id, input, output, type, status, timestamp, duration_ms}