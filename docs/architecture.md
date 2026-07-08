# Architecture Documentation

## Overview

Unified application combining piticlin/secretario.py (Python) and SDK-eventos-cadiz12 (Rust).

## Layers

1. **Presentation**: UI/Web interface (React/HTMX)
2. **Application**: Domain services (Rust) + AI orchestration (Python)
3. **Data**: SQLite storage + repositories
4. **External**: Mistral API integration

## Ports

- Rust Backend: 9090
- Python Service: 9000
- UI Frontend: 3000

## Communication

- Internal: REST/JSON between Rust and Python
- External: HTTP to Mistral API
