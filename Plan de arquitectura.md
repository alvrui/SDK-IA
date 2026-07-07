## **Plan de Arquitectura de la Aplicación Unificada**

### **Visión General**

La nueva aplicación integrará **`secretario.py`** (gestión de agentes Mistral) y **`SDK-eventos-cadiz12`** (editor narrativo) en un único sistema coherente, eliminando la dependencia externa entre ambos y optimizando la experiencia de desarrollo.

La arquitectura propuesta sigue un **modelo en capas con separación estricta de responsabilidades**, manteniendo los principios de:
- **Modularidad**: Cada componente tiene una responsabilidad clara y única
- **Extensibilidad**: Diseño abierto para añadir nuevos tipos de contenido, agentes y flujos de trabajo
- **Mantenibilidad**: Código limpio, documentado y con pruebas automatizadas
- **Rendimiento**: Uso eficiente de recursos y comunicaciones asíncronas

---

### **Arquitectura en Capas**

```
┌─────────────────────────────────────────────────────────────────────┐
│                           CAPA DE PRESENTACIÓN                          │
│  ┌─────────────────┐  ┌─────────────────────────────────────────────┐ │
│  │   UI Web        │  │           API REST/HTTP                        │ │
│  │  (React/HTMX)   │  │  - Endpoints para gestión de proyectos         │ │
│  │                 │  │  - Endpoints para interacción con agentes       │ │
│  │  - Editor       │  │  - WebSocket para comunicación en tiempo real │ │
│  │    narrativo    │  │  - Autenticación y autorización                │ │
│  │  - Consola de   │  │                                                 │ │
│  │    agentes      │  │                                                 │ │
│  │  - Visualizador │  │                                                 │ │
│  │    de logs      │  │                                                 │ │
│  └─────────────────┘  └─────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────────────┐
│                         CAPA DE APLICACIÓN                             │
│  ┌─────────────────────────┐  ┌─────────────────────────────────────┐ │
│  │   Servicios de Dominio    │  │       Servicios de Integración IA      │ │
│  │  (Rust)                  │  │       (Python/Rust)                     │ │
│  │  - Gestión de proyectos  │  │  - Orchestración de agentes Mistral    │ │
│  │  - Validación de         │  │  - Gestión de conversaciones           │ │
│  │    estructuras           │  │  - Normalización de respuestas         │ │
│  │  - Selección de eventos  │  │  - Persistencia de logs                 │ │
│  │  - Generación de         │  │  - Cache de respuestas frecuentes      │ │
│  │    narrativas            │  │                                             │ │
│  │  - Serialización/        │  │                                             │ │
│  │    deserialización        │  │                                             │ │
│  └─────────────────────────┘  └─────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────────────┐
│                         CAPA DE DATOS                                  │
│  ┌─────────────────────┐  ┌─────────────────────────────────────────┐ │
│  │   Almacenamiento      │  │           Repositorios                     │ │
│  │  - SQLite             │  │  - Proyectos                              │ │
│  │    (proyectos,        │  │  - Agentes                                │ │
│  │     agentes, logs)    │  │  - Plantillas                             │ │
│  │  - Sistema de         │  │  - Catálogos (facciones, personajes, etc.) │ │
│  │    archivos           │  │  - Historial de operaciones               │ │
│  │    (JSON/YAML)        │  │                                             │ │
│  └─────────────────────┘  └─────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────────────┐
│                       CAPA DE INTEGRACIÓN EXTERNA                      │
│  ┌─────────────────────────────────────────────────────────────────┐ │
│  │  - API de Mistral (beta.conversations)                            │ │
│  │  - Webhooks para notificaciones                                   │ │
│  │  - Integración con sistemas de almacenamiento externo (opcional)   │ │
│  └─────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

---

### **Tecnologías y Stack Propuesto**

| Componente | Tecnología | Justificación |
|------------|------------|---------------|
| **Backend Principal** | Rust (Actix-web o Axum) | Rendimiento, seguridad de tipos, concurrency nativa |
| **Backend de IA** | Python (FastAPI) | Integración nativa con SDK de Mistral, ecosistema de ML |
| **Frontend** | React + TypeScript o HTMX + Hyperscript | Experiencia de usuario rica con menor complejidad |
| **Base de Datos** | SQLite (principal) + opcional PostgreSQL | Persistencia local simple, escalable si es necesario |
| **Comunicación Interna** | gRPC o REST/JSON | Eficiencia en comunicación entre módulos |
| **Gestión de Estado** | Redux (frontend) + Pattern Matching (Rust) | Estado predecible y manejable |
| **Logging** | structlog (Python) + tracing (Rust) | Trazabilidad completa de operaciones |
| **Testing** | pytest (Python) + cargo test (Rust) | Cobertura de pruebas en ambos lenguajes |

---

### **Estructura de Directorios Propuesta**

```
/cadiz12-unified
├── Cargo.toml                    # Configuración principal del proyecto Rust
├── pyproject.toml                # Configuración del proyecto Python
├── README.md                     # Documentación principal
├── docs/                         # Documentación técnica
│   ├── architecture.md
│   ├── api-contract.md
│   └── development.md
├── src/                          # Código fuente Rust
│   ├── main.rs                   # Punto de entrada principal
│   ├── config.rs                 # Configuración de la aplicación
│   ├── domain/                   # Modelo de dominio
│   │   ├── mod.rs
│   │   ├── structures.rs         # ScriptElementBase, EventOutcomePrototype, etc.
│   │   ├── enums.rs              # Tipos enumerados del dominio
│   │   └── ids.rs                 # Sistema de identificadores
│   ├── services/                 # Servicios de aplicación
│   │   ├── project.rs             # Gestión de proyectos
│   │   ├── validation.rs          # Validación de estructuras
│   │   ├── selection.rs           # Selección de eventos
│   │   ├── generation.rs          # Generación de narrativas
│   │   └── persistence.rs         # Persistencia de datos
│   ├── api/                      # Capa API
│   │   ├── mod.rs
│   │   ├── handlers.rs            # Manejadores de endpoints
│   │   ├── routes.rs              # Definición de rutas
│   │   └── middleware.rs          # Middleware (autenticación, logging)
│   └── web/                      # Frontend embebido (opcional)
│       └── assets/                # Recursos estáticos
├── python/                       # Código Python
│   ├── secretario/                # Módulo de gestión de agentes
│   │   ├── __init__.py
│   │   ├── main.py                # Servidor FastAPI
│   │   ├── agents.py              # Gestión de agentes
│   │   ├── mistral_client.py      # Cliente de Mistral
│   │   ├── storage.py             # Persistencia (SQLite)
│   │   ├── models.py              # Modelos de datos
│   │   └── schemas.py             # Esquemas Pydantic
│   └── tests/                     # Pruebas unitarias
├── ui/                           # Frontend (si se usa separado)
│   ├── package.json
│   ├── src/
│   │   ├── App.tsx
│   │   ├── components/
│   │   ├── hooks/
│   │   └── styles/
│   └── public/
├── data/                         # Datos persistentes
│   ├── projects/                  # Proyectos guardados
│   ├── agents.db                  # Base de datos SQLite de agentes
│   └── logs/                      # Historial de operaciones
├── scripts/                      # Scripts de utilidad
│   ├── build.py
│   ├── deploy.py
│   └── test_integration.py
└── tests/                        # Pruebas de integración
```

---

### **Flujo de Datos Principal**

```
┌─────────────┐    ┌─────────────────┐    ┌─────────────────────┐    ┌─────────────────┐
│   Usuario    │───▶│   UI Web        │───▶│   API REST/HTTP      │───▶│   Servicio de    │
│              │    │                 │    │                     │    │   Dominio        │
└─────────────┘    └─────────────────┘    └─────────────────────┘    └─────────────────┘
                                                                           │
                                                                           ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                                 │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────────┐      │
│  │  Validación      │    │  Selección de    │    │  Generación de       │      │
│  │  de estructura   │───▶│  eventos         │───▶│  narrativas          │      │
│  └─────────────────┘    └─────────────────┘    └─────────────────────┘      │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────┘
                                                                           │
                                                                           ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                                 │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────────┐      │
│  │  Servicio de     │    │  Integración     │    │  Persistencia        │      │
│  │  Agentes IA      │───▶│  con Mistral     │───▶│  (SQLite)            │      │
│  └─────────────────┘    └─────────────────┘    └─────────────────────┘      │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────┐
│  Respuesta al Usuario     │
└─────────────────────────┘
```

---

### **Integración de los Componentes Existentes**

#### **1. Integración de `secretario.py`**
- **Migrar a módulo Python independiente** dentro de la nueva estructura
- **Refactorizar** para separar:
  - Lógica de agentes (`agents.py`)
  - Cliente de Mistral (`mistral_client.py`)
  - Persistencia (`storage.py`)
  - Servidor HTTP (`main.py`)
- **Exponer API REST** estandarizada para:
  - Gestión de agentes (CRUD)
  - Envío de mensajes
  - Consulta de logs
  - Estado de conversaciones

#### **2. Integración de `SDK-eventos-cadiz12`**
- **Mantener el crate Rust** como núcleo de dominio
- **Extender funcionalidades**:
  - Integración directa con el módulo de agentes (sin llamadas HTTP externas)
  - UI unificada que combine:
    - Editor narrativo
    - Consola de agentes
    - Visualizador de logs
- **Añadir nuevas capacidades**:
  - Gestión de proyectos completos
  - Validación en tiempo real
  - Previsualización de cambios

#### **3. Capa de Comunicación Interna**
Implementar un **protocolo de comunicación interno** entre los módulos Rust y Python:

**Opción A: gRPC (Recomendado)**
- Definir protos para:
  - Solicitudes de generación de contenido
  - Respuestas de agentes
  - Gestión de estado
- Ventajas: Tipado fuerte, eficiencia, soporte multi-lenguaje

**Opción B: REST/JSON interno**
- Endpoints internos expuestos por el módulo Python
- Llamadas desde Rust usando reqwest
- Ventajas: Simplicidad, compatibilidad con existente

**Opción C: FFI (Foreign Function Interface)**
- Exponer funciones Rust a Python mediante PyO3
- Ventajas: Máximo rendimiento, integración nativa
- Desventajas: Complejidad en la implementación

**Recomendación**: **Opción B (REST/JSON interno)** para la primera versión, con migración progresiva a gRPC.

---

### **Endpoints API Propuestos**

| Método | Endpoint | Descripción | Componente |
|--------|----------|-------------|------------|
| GET | `/api/projects` | Listar proyectos | Rust |
| POST | `/api/projects` | Crear proyecto | Rust |
| GET | `/api/projects/{id}` | Obtener proyecto | Rust |
| PUT | `/api/projects/{id}` | Actualizar proyecto | Rust |
| DELETE | `/api/projects/{id}` | Eliminar proyecto | Rust |
| GET | `/api/projects/{id}/validate` | Validar proyecto | Rust |
| POST | `/api/projects/{id}/generate` | Generar contenido | Rust |
| GET | `/api/agents` | Listar agentes | Python |
| POST | `/api/agents` | Crear agente | Python |
| GET | `/api/agents/{name}` | Obtener agente | Python |
| DELETE | `/api/agents/{name}` | Eliminar agente | Python |
| POST | `/api/agents/{name}/message` | Enviar mensaje | Python |
| GET | `/api/agents/{name}/conversation` | Obtener conversación | Python |
| GET | `/api/logs` | Obtener logs | Python |
| POST | `/api/ai/narrative` | Generar narrativa | Rust (delegando a Python) |
| POST | `/api/ai/story-elements` | Generar story elements | Rust (delegando a Python) |
| POST | `/api/ai/event` | Generar evento | Rust (delegando a Python) |
| POST | `/api/ai/review` | Revisar contenido | Rust (delegando a Python) |
| GET | `/api/health` | Estado de salud | Ambos |
| GET | `/api/version` | Versión de la API | Ambos |

---

### **Estructura de Datos Unificada**

#### **Modelo de Proyecto**
```typescript
interface Project {
  id: string;
  title: string;
  summary: string;
  act: ActType;
  tone: ToneType;
  historical_scope: HistoricalScope;
  spaces: Space[];
  factions: Faction[];
  stakes: Stake[];
  tags: Tag[];
  story_elements: StoryElement[];
  events: Event[];
  ai_history: AIAction[];
  settings: ProjectSettings;
  metadata: ProjectMetadata;
}

interface ProjectSettings {
  agents: Record<SectionType, string>; // Mapeo sección → nombre de agente
  validation_rules: ValidationConfig;
  generation_params: GenerationConfig;
}

interface ProjectMetadata {
  created_at: string;
  updated_at: string;
  version: string;
  author: string;
}
```

#### **Modelo de Agente**
```typescript
interface Agent {
  id: string;
  name: string;
  description: string;
  role: AgentRole;
  conversation_id: string | null;
  configuration: AgentConfig;
  created_at: string;
  updated_at: string;
  last_used_at: string;
}

type AgentRole =
  | 'CoordinadorNarrativo'
  | 'ContextoHistorico'
  | 'ContextoJuego'
  | 'DiseñadorDeStoryElements'
  | 'DiseñadorDeEventos'
  | 'RevisorNarrativo'
  | 'EditorDeExportacion'
  | 'Custom';
```

#### **Modelo de Mensaje/Log**
```typescript
interface MessageLog {
  id: string;
  agent_id: string;
  agent_name: string;
  conversation_id: string;
  input: string;
  output: string;
  status: 'success' | 'error' | 'pending';
  type: 'user' | 'agent' | 'system';
  timestamp: string;
  duration_ms: number;
  metadata: Record<string, any>;
}
```

---

### **Mecanismo de Validación y Generación**

**Flujo de trabajo para generación de contenido:**

1. **Solicitud de usuario** → UI envía petición a endpoint Rust
2. **Validación previa** (Rust):
   - Verificar estructura del proyecto
   - Validar parámetros
   - Comprobar permisos
3. **Selección de agente** (Rust):
   - Consultar configuración del proyecto
   - Resolver nombre de agente para la sección
4. **Llamada a servicio de agentes** (Rust → Python):
   - Construir prompt contextualizado
   - Enviar solicitud al módulo Python
5. **Procesamiento IA** (Python):
   - Enviar mensaje a Mistral
   - Normalizar respuesta
   - Registrar en logs
6. **Post-procesamiento** (Rust):
   - Parsear respuesta JSON
   - Validar contra esquema de dominio
   - Transformar a estructuras nativas
7. **Devolver propuesta** → UI muestra borrador
8. **Aplicación opcional** → Usuario acepta/rechaza/modifica

---

### **Persistencia de Datos**

**Estrategia de almacenamiento:**

1. **SQLite** (principal):
   - Tabla `projects`: Metadatos de proyectos
   - Tabla `agents`: Configuración de agentes
   - Tabla `conversations`: Historial de conversaciones
   - Tabla `logs`: Registro de operaciones
   - Tabla `app_state`: Estado de la aplicación

2. **Sistema de archivos**:
   - `/data/projects/{id}/project.json`: Contenido completo del proyecto
   - `/data/projects/{id}/backups/`: Copias de seguridad
   - `/data/projects/{id}/exports/`: Exportaciones

3. **Cache en memoria**:
   - Proyectos abiertos recientemente
   - Agentes frecuentemente usados
   - Respuestas IA cacheadas (configurable)

---

### **Gestión de Configuración**

**Archivos de configuración:**

```toml
# config.toml
[server]
host = "0.0.0.0"
port = 8080
debug = true
cors_origins = ["*"]

[database]
path = "./data/app.db"
backup_interval = "24h"

[mistral]
api_key = "${MISTRAL_API_KEY}"
api_url = "https://api.mistral.ai/v1"
timeout = 30

[ai]
max_tokens = 4096
temperature = 0.7
model = "mistral-large"

[logging]
level = "info"
file = "./data/logs/app.log"
max_size = "10MB"
rotation = "daily"
```

---

## **Análisis Funcional de la Aplicación Resultante**

---

### **Funcionalidades Principales**

#### **1. Gestión de Proyectos Narrativos**
- **Creación**: Inicializar nuevos proyectos con configuración base
- **Edición**: Modificar todos los aspectos de un proyecto (narrativa, eventos, personajes, etc.)
- **Validación**: Verificar coherencia de datos según reglas de dominio
- **Versiónado**: Historial de cambios con capacidad de rollback
- **Exportación**: Generar archivos compatibles con el motor de Cádiz12
- **Importación**: Cargar proyectos existentes

#### **2. Gestión de Agentes IA**
- **Registro**: Añadir nuevos agentes con roles específicos
- **Configuración**: Personalizar parámetros de cada agente (temperatura, modelo, etc.)
- **Monitoreo**: Visualizar estado y uso de cada agente
- **Pruebas**: Enviar mensajes de prueba para verificar funcionamiento
- **Historial**: Consultar conversaciones previas

#### **3. Generación Asistida por IA**
- **Narrativas**: Generar textos narrativos coherentes con el contexto histórico
- **Story Elements**: Crear elementos de historia (personajes, escenarios, recursos)
- **Eventos**: Generar eventos con consecuencias definidas
- **Revisión**: Validar y mejorar contenido existente
- **Sugerencias**: Propuestas contextualizadas para cada sección

#### **4. Interfaz de Usuario Unificada**
- **Editor visual**: Interfaz intuitiva para editar proyectos
- **Consola de agentes**: Interfaz para interactuar directamente con agentes
- **Visualizador de logs**: Historial detallado de todas las operaciones
- **Previsualización**: Vista previa de cambios antes de aplicarlos
- **Dashboard**: Panel de control con métricas y estado del sistema

#### **5. Sistema de Logs y Trazabilidad**
- **Registro completo**: Todas las interacciones con IA quedan registradas
- **Búsqueda avanzada**: Filtrar logs por agente, proyecto, fecha, etc.
- **Exportación**: Descargar logs en formatos JSON/CSV
- **Auditoría**: Seguimiento de quién hizo qué y cuándo

---

### **Ventajas de la Aplicación Unificada**

| Aspecto | Antes (Separado) | Después (Unificado) |
|---------|------------------|---------------------|
| **Integración** | Llamadas HTTP externas entre aplicaciones | Comunicación interna directa |
| **Experiencia de Usuario** | Interfaces separadas, flujo fragmentado | Interfaz única, flujo coherente |
| **Rendimiento** | Latencia de red entre servicios | Comunicación local, menor latencia |
| **Mantenimiento** | Dos repositorios, configuraciones separadas | Un solo código base, configuración centralizada |
| **Despliegue** | Múltiples procesos, puertos, dependencias | Un solo proceso, gestión simplificada |
| **Seguridad** | Exposición de endpoints internos | Encapsulamiento de la lógica interna |
| **Consistencia** | Riesgo de divergencia entre versiones | Versión única, actualizaciones coordinadas |
| **Debugging** | Logs distribuidos en dos sistemas | Logs centralizados y correlacionados |

---

### **Riesgos y Mitigaciones**

| Riesgo | Probabilidad | Impacto | Mitigación |
|--------|--------------|---------|------------|
| **Complejidad en integración Rust-Python** | Media | Alto | Usar protocolo REST/JSON interno bien definido, pruebas exhaustivas |
| **Rendimiento en comunicación inter-módulos** | Baja | Medio | Optimizar llamadas, usar cache, considerar gRPC |
| **Mantenimiento de dos lenguajes** | Alta | Medio | Documentación clara, equipo con conocimientos en ambos, CI/CD robusta |
| **Divergencia en modelos de datos** | Media | Alto | Definir esquemas comunes en JSON Schema, validación estricta |
| **Problemas de dependencias** | Media | Medio | Usar herramientas de gestión de dependencias (Cargo, Poetry), contenedores |
| **Dificultad en debugging** | Media | Medio | Logging centralizado, correlación de IDs en logs, trazas distribuidas |
| **Escalabilidad limitada con SQLite** | Baja | Medio | Diseñar para migración futura a PostgreSQL, usar patrones de repositorio |

---

### **Fases de Implementación**

#### **Fase 1: Preparación (2 semanas)**
- [ ] Definir contratos de API internos
- [ ] Configurar estructura de directorios
- [ ] Migrar `secretario.py` a módulo Python dentro de la nueva estructura
- [ ] Configurar entorno de desarrollo (Docker, scripts de build)
- [ ] Establecer pipeline de CI/CD básico

#### **Fase 2: Integración Básica (3 semanas)**
- [ ] Implementar capa de comunicación entre Rust y Python (REST/JSON)
- [ ] Crear endpoints API principales
- [ ] Integrar UI básica (versión mínima viable)
- [ ] Implementar persistencia unificada (SQLite)
- [ ] Añadir logging centralizado

#### **Fase 3: Funcionalidades Nucleares (4 semanas)**
- [ ] Implementar gestión de proyectos completa
- [ ] Integrar generación asistida por IA
- [ ] Añadir validación de dominio
- [ ] Implementar sistema de logs
- [ ] Crear consola de agentes unificada

#### **Fase 4: Mejoras y Optimización (3 semanas)**
- [ ] Optimizar rendimiento de comunicación inter-módulos
- [ ] Añadir cache de respuestas IA
- [ ] Implementar versiónado de proyectos
- [ ] Mejorar UI con funcionalidades avanzadas
- [ ] Añadir sistema de backup automático

#### **Fase 5: Pruebas y Despliegue (2 semanas)**
- [ ] Pruebas unitarias completas
- [ ] Pruebas de integración
- [ ] Pruebas de rendimiento
- [ ] Documentación final
- [ ] Despliegue en entorno de producción

**Duración total estimada**: 14 semanas (3.5 meses)

---
---
## **Recomendaciones Finales**

1. **Priorizar la compatibilidad**: Asegurar que la nueva aplicación pueda importar proyectos y configuraciones existentes de ambas aplicaciones originales.

2. **Mantener la separación de responsabilidades**: Aunque unificados en una sola aplicación, los módulos de Rust (dominio) y Python (IA) deben mantener su independencia funcional.

3. **Inversión en pruebas**: Implementar pruebas automatizadas desde el principio para evitar regresiones en la integración.

4. **Documentación exhaustiva**: Documentar no solo el código, sino también:
   - Contratos de API
   - Flujos de trabajo
   - Decisiones arquitectónicas
   - Guías de despliegue y operación

5. **Monitorización**: Implementar métricas y alertas para:
   - Tiempo de respuesta de IA
   - Uso de tokens de Mistral
   - Errores en validación
   - Rendimiento de la aplicación

6. **Escalabilidad futura**: Diseñar la arquitectura pensando en:
   - Migración a base de datos relacional (PostgreSQL)
   - Despliegue en contenedores (Docker)
   - Escalado horizontal (si es necesario)
   - Integración con otros servicios de IA

7. **Experiencia de desarrollador**: Proporcionar:
   - Scripts de desarrollo claros
   - Entorno de desarrollo estandarizado (dev containers)
   - Herramientas de debugging integradas

---
---
Este plan proporciona una hoja de ruta clara para unificar las dos aplicaciones manteniendo sus fortalezas individuales mientras se resuelven los problemas de integración y experiencia de usuario actuales.
