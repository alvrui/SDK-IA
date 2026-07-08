# SDK-IA Compilation Guide

## Quick Fix

Before compiling, run:
rm -f src/logging.rs

This removes the file that conflicts with the src/logging/ directory.

## Rust Backend

### Build
cargo build

### Run
cargo run

## Python Service

### Install
cd python
pip install -r requirements.txt

### Run
python -m secreto.main

## UI

### Install
cd ui
npm install

### Run
npm run dev

## Database
python scripts/init_database.py

## Logs
- Rust: data/logs/rust-backend.log
- Python: data/logs/python-service.log
- API: GET /api/v1/internal/logs
