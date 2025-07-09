.PHONY: help install install-rust install-npm build-web build build-release clean dev test

# Default target
help:
	@echo "Available targets:"
	@echo "  install        - Install all dependencies (Rust and npm)"
	@echo "  install-rust   - Install Rust dependencies"
	@echo "  install-npm    - Install npm dependencies for web frontend"
	@echo "  build-web      - Build the web/netdrop React project"
	@echo "  build          - Build the Rust project in debug mode"
	@echo "  build-release  - Build the Rust project in release mode"
	@echo "  dev            - Start development server for web frontend"
	@echo "  test           - Run tests"
	@echo "  clean          - Clean build artifacts"

# Install all dependencies
install: install-rust install-npm

# Install Rust dependencies
install-rust:
	@echo "Installing Rust dependencies..."
	cargo fetch

# Install npm dependencies for web frontend
install-npm:
	@echo "Installing npm dependencies for web frontend..."
	cd web/netdrop && npm install

# Build the web/netdrop React project
build-web:
	@echo "Building web frontend..."
	cd web/netdrop && npm run build

# Build the Rust project in debug mode
build:
	@if [ ! -d "web/netdrop/dist" ]; then \
		echo "Web build not found, building web frontend first..."; \
		$(MAKE) build-web; \
	else \
		echo "Web build found, skipping frontend build..."; \
	fi
	@echo "Building Rust project (debug)..."
	cargo build

# Build the Rust project in release mode
build-release:
	@if [ ! -d "web/netdrop/dist" ]; then \
		echo "Web build not found, building web frontend first..."; \
		$(MAKE) build-web; \
	else \
		echo "Web build found, skipping frontend build..."; \
	fi
	@echo "Building Rust project (release)..."
	cargo build --release

# Development server for web frontend
dev:
	@echo "Starting development server for web frontend..."
	cd web/netdrop && npm run dev

# Run tests
test:
	@echo "Running Rust tests..."
	cargo test
	@echo "Running web frontend linting..."
	cd web/netdrop && npm run lint

# Clean build artifacts
clean:
	@echo "Cleaning Rust build artifacts..."
	cargo clean
	@echo "Cleaning web frontend build artifacts..."
	cd web/netdrop && rm -rf dist node_modules
