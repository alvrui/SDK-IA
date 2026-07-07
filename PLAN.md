# Plan de Ejecucion: Unificacion de piticlin/secretario.py y SDK-eventos-cadiz12

**Estado**: ACTIVO - Seguir al pie de la letra
**Fecha de creacion**: 7 de julio de 2026
**Objetivo**: Crear una sola aplicacion unificada a partir de las dos aplicaciones actuales que trabajan de forma separada.

---

## Contexto

Actualmente existen dos aplicaciones interconectadas pero separadas:
1. piticlin/secretario.py: Servicio Python que gestiona agentes de Mistral, persiste conversaciones y expone una interfaz HTTP local
2. SDK-eventos-cadiz12: Proyecto Rust con UI web local para generar narrativas, story elements y eventos

Ambas forman una arquitectura en capas donde secretario.py orquestra agentes y conversaciones, mientras que SDK-eventos-cadiz12 ofrece el dominio narrativo y una UI editorial.

Problema: Comunicacion externa entre aplicaciones, divergencia de versiones, experiencia de usuario fragmentada.

Solucion: Aplicacion unificada con comunicacion interna directa, interfaz unica y flujo coherente.

---

## Fases de Implementacion

### Fase 1: Preparacion (2 semanas)

Objetivo: Establecer las bases del proyecto unificado

- [ ] Definir contratos de API internos
- [ ] Configurar estructura de directorios
- [ ] Migrar secretario.py a modulo Python dentro de la nueva estructura
- [ ] Configurar entorno de desarrollo (Docker, scripts de build)
- [ ] Establecer pipeline de CI/CD basico

Entregables:
- Documento de contratos de API
- Estructura de directorios funcional
- Modulo Python integrado
- Entorno de desarrollo operativo
- Pipeline CI/CD configurado

Fecha estimada: 7 de julio de 2026 - 21 de julio de 2026

---

### Fase 2: Integracion Basica (3 semanas)

Objetivo: Conectar los componentes principales y crear una version minima viable

- [ ] Implementar capa de comunicacion entre Rust y Python (REST/JSON)
- [ ] Crear endpoints API principales
- [ ] Integrar UI basica (version minima viable)
- [ ] Implementar persistencia unificada (SQLite)
- [ ] Anadir logging centralizado

Entregables:
- Comunicacion inter-modulos funcional
- API con endpoints principales
- UI basica operativa
- Persistencia funcionando
- Sistema de logs centralizado

Fecha estimada: 22 de julio de 2026 - 11 de agosto de 2026

---

### Fase 3: Funcionalidades Nucleares (4 semanas)

Objetivo: Implementar las funcionalidades principales del sistema

- [ ] Implementar gestion de proyectos completa
- [ ] Integrar generacion asistida por IA
- [ ] Anadir validacion de dominio
- [ ] Implementar sistema de logs
- [ ] Crear consola de agentes unificada

Entregables:
- Gestion de proyectos funcional
- Generacion IA integrada
- Validacion de dominio operativa
- Sistema de logs completo
- Consola de agentes unificada

Fecha estimada: 12 de agosto de 2026 - 8 de septiembre de 2026

---

### Fase 4: Mejoras y Optimizacion (3 semanas)

Objetivo: Optimizar el rendimiento y anadir funcionalidades avanzadas

- [ ] Optimizar rendimiento de comunicacion inter-modulos
- [ ] Anadir cache de respuestas IA
- [ ] Implementar versionado de proyectos
- [ ] Mejorar UI con funcionalidades avanzadas
- [ ] Anadir sistema de backup automatico

Entregables:
- Comunicacion inter-modulos optimizada
- Cache de respuestas IA implementado
- Versionado de proyectos funcional
- UI mejorada
- Sistema de backup automatico

Fecha estimada: 9 de septiembre de 2026 - 29 de septiembre de 2026

---

### Fase 5: Pruebas y Despliegue (2 semanas)

Objetivo: Asegurar calidad y desplegar en produccion

- [ ] Pruebas unitarias completas
- [ ] Pruebas de integracion
- [ ] Pruebas de rendimiento
- [ ] Documentacion final
- [ ] Despliegue en entorno de produccion

Entregables:
- Suite de pruebas completa
- Documentacion tecnica y de usuario
- Aplicacion desplegada y operativa

Fecha estimada: 30 de septiembre de 2026 - 14 de octubre de 2026

---

## Cronograma General

| Fase | Duracion | Fecha Inicio | Fecha Fin |
|------|----------|--------------|-----------|
| Fase 1: Preparacion | 2 semanas | 7 de julio de 2026 | 21 de julio de 2026 |
| Fase 2: Integracion Basica | 3 semanas | 22 de julio de 2026 | 11 de agosto de 2026 |
| Fase 3: Funcionalidades Nucleares | 4 semanas | 12 de agosto de 2026 | 8 de septiembre de 2026 |
| Fase 4: Mejoras y Optimizacion | 3 semanas | 9 de septiembre de 2026 | 29 de septiembre de 2026 |
| Fase 5: Pruebas y Despliegue | 2 semanas | 30 de septiembre de 2026 | 14 de octubre de 2026 |

Duracion total estimada: 14 semanas (3.5 meses)

---

## Arquitectura de Referencia

### Estructura de Directorios

/cadiz12-unified
├── Cargo.toml
├── pyproject.toml
├── README.md
├── PLAN.md
├── docs/
│   ├── architecture.md
│   ├── api-contract.md
│   └── development.md
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── structures.rs
│   │   ├── enums.rs
│   │   └── ids.rs
│   ├── services/
│   │   ├── project.rs
│   │   ├── validation.rs
│   │   ├── selection.rs
│   │   ├── generation.rs
│   │   └── persistence.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── handlers.rs
│   │   ├── routes.rs
│   │   └── middleware.rs
│   └── web/
│       └── assets/
├── python/
│   ├── secreto/
│   │   ├── __init__.py
│   │   ├── main.py
│   │   ├── agents.py
│   │   ├── mistral_client.py
│   │   ├── storage.py
│   │   ├── models.py
│   │   └── schemas.py
│   └── tests/
├── ui/
│   ├── package.json
│   ├── src/
│   │   ├── App.tsx
│   │   ├── components/
│   │   ├── hooks/
│   │   └── styles/
│   └── public/
├── data/
│   ├── projects/
│   ├── agents.db
│   └── logs/
├── scripts/
│   ├── build.py
│   ├── deploy.py
│   └── test_integration.py
└── tests/

### Tecnologias

| Componente | Tecnologia |
|------------|------------|
| Backend Principal | Rust (Actix-web/Axum) |
| Backend de IA | Python (FastAPI) |
| Frontend | React + TypeScript / HTMX + Hyperscript |
| Base de Datos | SQLite (principal) + opcional PostgreSQL |
| Comunicacion Interna | REST/JSON (inicial) -> gRPC (futuro) |
| Logging | structlog (Python) + tracing (Rust) |
| Testing | pytest (Python) + cargo test (Rust) |

---

## Principios a Seguir

1. Separacion de responsabilidades: Mantener clara la division entre dominio (Rust) y orquestacion IA (Python)
2. Contratos explicitos: Todos los intercambios entre modulos deben estar claramente definidos
3. Pruebas desde el principio: Implementar pruebas automatizadas para cada componente
4. Documentacion continua: Documentar decisiones, contratos y flujos de trabajo
5. Compatibilidad hacia atras: Asegurar que se puedan importar proyectos y configuraciones existentes

---

## Estado Actual de Seguimiento

Ultima actualizacion: 7 de julio de 2026
Fase actual: Fase 1: Preparacion
Proximos pasos: Iniciar con la definicion de contratos de API internos

---

## Notas Adicionales

- Este plan debe seguirse AL PIE DE LA LETRA
- Cualquier desviacion debe ser justificada y documentada
- El usuario sera informado de cualquier bloqueo o decision critica
- Se priorizara la calidad sobre la velocidad de entrega