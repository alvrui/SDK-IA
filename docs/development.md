# Development Guide

## Prerequisites

- Python 3.13.5
- Rust 1.96
- Node.js 18+ (for UI)
- Debian Linux

## Setup

```bash
git clone https://github.com/alvrui/SDK-IA.git
cd SDK-IA

# Python
python -m venv .venv
source .venv/bin/activate
pip install -e .

# Rust
# (managed by rustup)

# UI
cd ui
npm install
cd ..
```

## Running

```bash
# Rust backend
cargo run

# Python service
cd python
python -m secreto.main

# UI
cd ui
npm run dev
```

## Testing

```bash
# Rust tests
cargo test

# Python tests
cd python
pytest

# Integration tests
python scripts/test_integration.py
```

## Structure

See PLAN.md for detailed project structure.
