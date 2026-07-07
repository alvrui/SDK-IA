# SDK-IA: Sistema Unificado de Desarrollo Narrativo con IA

## Descripcion General

SDK-IA es un proyecto unificado que integra dos aplicaciones previamente separadas:
- piticlin/secretario.py: Servicio Python para gestion de agentes Mistral
- SDK-eventos-cadiz12: Proyecto Rust para generacion de narrativas, story elements y eventos

El objetivo es crear una unica aplicacion que combine la orquestacion de agentes de IA con el modelado narrativo para el proyecto Cadiz 1812.

## Contexto y Motivacion

### Problema Inicial
Originalmente existian dos aplicaciones interconectadas pero separadas que trabajaban en el proyecto torre.

### Solucion Propuesta
Aplicacion unificada con comunicacion interna directa, interfaz unica y flujo coherente.

## Arquitectura del Sistema

### Tecnologias
- Backend Principal: Rust (Actix-web/Axum)
- Backend de IA: Python (FastAPI)
- Frontend: React + TypeScript / HTMX + Hyperscript
- Base de Datos: SQLite (principal) + opcional PostgreSQL
- Comunicacion Interna: REST/JSON

## Estructura del Proyecto
Consultar [PLAN.md](PLAN.md) para la estructura de directorios detallada.

## Flujos de Trabajo
1. Todas las validaciones se realizan en el entorno de desarrollo
2. El codigo que funciona sin fallos se promueve a entorno de pruebas local
3. Finalmente, se despliega al entorno de produccion local

## Documentacion Adicional
- [Plan de Ejecucion](PLAN.md) - Plan detallado con fases, cronograma y tareas