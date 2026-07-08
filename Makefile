# Makefile for SDK-IA project
# Provides common development commands

.PHONY: help build test run clean lint format check

# Default target
help:
	@echo "SDK-IA Development Commands:"
	@echo ""
	@echo "  make build          - Build all components"
	@echo "  make test           - Run all tests"
	@echo "  make run            - Start all services"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make lint           - Run linters"
	@echo "  make format         - Format code"
	@echo "  make check          - Check dependencies"
	@echo "  make setup          - Setup development environment"
	@echo ""
	@echo "Individual components:"
	@echo "  make build-rust     - Build Rust backend"
	@echo "  make build-python   - Install Python dependencies"
	@echo "  make build-ui       - Build UI frontend"
	@echo ""
	@echo "  make test-rust      - Run Rust tests"
	@echo "  make test-python    - Run Python tests"
	@echo "  make test-integration - Run integration tests"
	@echo ""
	@echo "  make run-rust       - Start Rust backend"
	@echo "  make run-python     - Start Python service"
	@echo "  make run-ui         - Start UI frontend"

# ============================================================================
# Build Commands
# ============================================================================

build: build-rust build-python build-ui

build-rust:
	@echo "Building Rust backend..."
	cd src && cargo build --release
	@echo "Rust backend built successfully"

build-python:
	@echo "Installing Python dependencies..."
	cd python && pip install -e .
	@echo "Python dependencies installed successfully"

build-ui:
	@echo "Building UI frontend..."
	cd ui && npm install && npm run build
	@echo "UI frontend built successfully"

# ============================================================================
# Test Commands
# ============================================================================

test: test-rust test-python test-integration

test-rust:
	@echo "Running Rust tests..."
	cd src && cargo test
	@echo "Rust tests completed"

test-python:
	@echo "Running Python tests..."
	cd python && pytest
	@echo "Python tests completed"

test-integration:
	@echo "Running integration tests..."
	python scripts/test_integration.py
	@echo "Integration tests completed"

# ============================================================================
# Run Commands
# ============================================================================

run: run-rust run-python run-ui

run-rust:
	@echo "Starting Rust backend on port 9090..."
	cd src && cargo run --release

run-python:
	@echo "Starting Python service on port 9000..."
	cd python && python -m secreto.main

run-ui:
	@echo "Starting UI frontend on port 3000..."
	cd ui && npm run dev

# ============================================================================
# Clean Commands
# ============================================================================

clean: clean-rust clean-python clean-ui

clean-rust:
	@echo "Cleaning Rust build..."
	cd src && cargo clean
	@echo "Rust build cleaned"

clean-python:
	@echo "Cleaning Python artifacts..."
	cd python && rm -rf __pycache__ *.pyc *.egg-info
	@echo "Python artifacts cleaned"

clean-ui:
	@echo "Cleaning UI build..."
	cd ui && rm -rf node_modules dist
	@echo "UI build cleaned"

# ============================================================================
# Linting and Formatting
# ============================================================================

lint: lint-rust lint-python lint-ui

lint-rust:
	@echo "Linting Rust code..."
	cd src && cargo clippy --all-targets --all-features

lint-python:
	@echo "Linting Python code..."
	cd python && ruff check secreto

lint-ui:
	@echo "Linting UI code..."
	cd ui && npm run lint

format: format-rust format-python format-ui

format-rust:
	@echo "Formatting Rust code..."
	cd src && cargo fmt

format-python:
	@echo "Formatting Python code..."
	cd python && black secreto

format-ui:
	@echo "Formatting UI code..."
	cd ui && npm run format

# ============================================================================
# Setup and Check
# ============================================================================

setup:
	@echo "Setting up development environment..."
	python scripts/setup_environment.py
	@echo "Environment setup completed"

check:
	@echo "Checking dependencies..."
	python scripts/check_dependencies.py
	@echo "Dependency check completed"
