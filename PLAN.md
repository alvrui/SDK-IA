# Plan de Ejecución: Unificación de piticlin/secretario.py y SDK-eventos-cadiz12

**Estado**: ACTIVO - Seguir al pie de la letra
**Fecha de creación**: 7 de julio de 2026
**Objetivo**: Crear una sola aplicación unificada a partir de las dos aplicaciones actuales que trabajan de forma separada.

---

## Contexto

Actualmente existen dos aplicaciones interconectadas pero separadas:
1. **piticlin/secretario.py**: Servicio Python que gestiona agentes de Mistral, persiste conversaciones y expone una interfaz HTTP local
2. **SDK-eventos-cadiz12**: Proyecto Rust con UI web local para generar narrativas, story elements y eventos

Ambas forman una arquitectura en capas donde secretario.py orquesta agentes y conversaciones, mientras que SDK-eventos-cadiz12 ofrece el dominio narrativo y una UI editorial.

**Problema**: Comunicación externa entre aplicaciones, divergencia de versiones, experiencia de usuario fragmentada.

**Solución**: Aplicación unificada con comunicación interna directa, interfaz única y flujo coherente.

---

## Fases de Implementación

### Fase 1: Preparación (2 semanas)

**Objetivo**: Establecer las bases del proyecto unificado

- [ ] Definir contratos de API internos
- [ ] Configurar estructura de directorios
- [ ] Migrar secretario.py a módulo Python dentro de la nueva estructura
- [ ] Configurar entorno de desarrollo (Docker, scripts de build)
- [ ] Establecer pipeline de CI/CD básico

**Entregables**:
- Documento de contratos de API
- Estructura de directorios funcional
- Módulo Python integrado
- Entorno de desarrollo operativo
- Pipeline CI/CD configurado

**Fecha estimada**: 7 de julio de 2026 - 21 de julio de 2026

---

### Fase 2: Integración Básica (3 semanas)

**Objetivo**: Conectar los componentes principales y crear una versión mínima viable

- [ ] Implementar capa de comunicación entre Rust y Python (REST/JSON)
- [ ] Crear endpoints API principales
- [ ] Integrar UI básica (versión mínima viable)
- [ ] Implementar persistencia unificada (SQLite)
- [ ] Anadir logging centralizado

**Entregables**:
- Comunicación inter-módulos funcional
- API con endpoints principales
- UI básica operativa
- Persistencia funcionando
- Sistema de logs centralizado

**Fecha estimada**: 22 de julio de 2026 - 11 de agosto de 2026

---

### Fase 3: Funcionalidades Nucleares (4 semanas)

**Objetivo**: Implementar las funcionalidades principales del sistema

- [ ] Implementar gestión de proyectos completa
- [ ] Integrar generación asistida por IA
- [ ] Anadir validación de dominio
- [ ] Implementar sistema de logs
- [ ] Crear consola de agentes unificada

**Entregables**:
- Gestión de proyectos funcional
- Generación IA integrada
- Validación de dominio operativa
- Sistema de logs completo
- Consola de agentes unificada

**Fecha estimada**: 12 de agosto de 2026 - 8 de septiembre de 2026

---

### Fase 4: Mejoras y Optimización (3 semanas)

**Objetivo**: Optimizar el rendimiento y anadir funcionalidades avanzadas

- [ ] Optimizar rendimiento de comunicación inter-módulos
- [ ] Anadir cache de respuestas IA
- [ ] Implementar versiónado de proyectos
- [ ] Mejorar UI con funcionalidades avanzadas
- [ ] Anadir sistema de backup automático

**Entregables**:
- Comunicación inter-módulos optimizada
- Cache de respuestas IA implementado
- Versiónado de proyectos funcional
- UI mejorada
- Sistema de backup automático

**Fecha estimada**: 9 de septiembre de 2026 - 29 de septiembre de 2026

---

### Fase 5: Pruebas y Despliegue (2 semanas)

**Objetivo**: Asegurar calidad y desplegar en producción

- [ ] Pruebas unitarias completas
- [ ] Pruebas de integración
- [ ] Pruebas de rendimiento
- [ ] Documentación final
- [ ] Despliegue en entorno de producción

**Entregables**:
- Suite de pruebas completa
- Documentación técnica y de usuario
- Aplicación desplegada y operativa

**Fecha estimada**: 30 de septiembre de 2026 - 14 de octubre de 2026

---

## Cronograma General

| Fase | Duración | Fecha Inicio | Fecha Fin |
|------|----------|--------------|-----------|
| Fase 1: Preparación | 2 semanas | 7 de julio de 2026 | 21 de julio de 2026 |
| Fase 2: Integración Básica | 3 semanas | 22 de julio de 2026 | 11 de agosto de 2026 |
| Fase 3: Funcionalidades Nucleares | 4 semanas | 12 de agosto de 2026 | 8 de septiembre de 2026 |
| Fase 4: Mejoras y Optimización | 3 semanas | 9 de septiembre de 2026 | 29 de septiembre de 2026 |
| Fase 5: Pruebas y Despliegue | 2 semanas | 30 de septiembre de 2026 | 14 de octubre de 2026 |

**Duración total estimada**: 14 semanas (3.5 meses)

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

### Tecnologías

| Componente | Tecnología |
|------------|------------|
| Backend Principal | Rust (Actix-web/Axum) |
| Backend de IA | Python (FastAPI) |
| Frontend | React + TypeScript / HTMX + Hyperscript |
| Base de Datos | SQLite (principal) + opcional PostgreSQL |
| Comunicación Interna | REST/JSON (inicial) -> gRPC (futuro) |
| Logging | structlog (Python) + tracing (Rust) |
| Testing | pytest (Python) + cargo test (Rust) |

---

## Principios a Seguir

1. Separación de responsabilidades: Mantener clara la división entre dominio (Rust) y orquestación IA (Python)
2. Contratos explícitos: Todos los intercambios entre módulos deben estar claramente definidos
3. Pruebas desde el principio: Implementar pruebas automatizadas para cada componente
4. Documentación continua: Documentar decisiones, contratos y flujos de trabajo
5. Compatibilidad hacia atrás: Asegurar que se puedan importar proyectos y configuraciones existentes

---

## Estado Actual de Seguimiento

**Última actualización**: 7 de julio de 2026
**Fase actual**: Fase 1: Preparación
**Próximos pasos**: Iniciar con la definición de contratos de API internos

---

## Notas Adicionales

- Este plan debe seguirse AL PIE DE LA LETRA
- Cualquier desviación debe ser justificada y documentada
- El usuario será informado de cualquier bloqueo o decisión crítica
- Se priorizará la calidad sobre la velocidad de entrega
