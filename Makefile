.PHONY: all build release run clean test install help docker docker-build docker-up docker-down docker-logs docker-attach docker-clean

# Default target
all: release

# Build debug version
build:
	@echo "Building debug version..."
	cargo build

# Build release version (optimized)
release:
	@echo "Building release version..."
	cargo build --release
	@echo ""
	@echo "Build successful!"
	@echo "Binary location: target/release/maximize"
	@echo ""
	@echo "To run:"
	@echo "  ./target/release/maximize"
	@echo ""
	@echo "Or with options:"
	@echo "  ./target/release/maximize --debug"
	@echo "  ./target/release/maximize --bind 127.0.0.1"

# Run debug version
run:
	cargo run

# Run release version
run-release:
	cargo run --release

# Run with debug logging
run-debug:
	cargo run --release -- --debug

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Run tests
test:
	cargo test

# Check code without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Lint code
lint:
	cargo clippy -- -D warnings

# Install binary to system
install: release
	@echo "Installing maximize to /usr/local/bin..."
	@sudo cp target/release/maximize /usr/local/bin/
	@echo "Installed successfully!"
	@echo "You can now run: maximize"

# Uninstall binary from system
uninstall:
	@echo "Uninstalling maximize..."
	@sudo rm -f /usr/local/bin/maximize
	@echo "Uninstalled successfully!"

# Docker commands
docker: docker-build docker-up

docker-build:
	@echo "Building Docker image..."
	docker-compose build

docker-up:
	@echo "Starting Docker container..."
	docker-compose up -d
	@echo ""
	@echo "Container started!"
	@echo "To authenticate, run: make docker-attach"
	@echo "To view logs, run: make docker-logs"

docker-down:
	@echo "Stopping Docker container..."
	docker-compose down

docker-logs:
	@echo "Viewing Docker logs (Ctrl+C to exit)..."
	docker-compose logs -f

docker-attach:
	@echo "Attaching to container for authentication..."
	@echo "After login, press Ctrl+P then Ctrl+Q to detach"
	@sleep 2
	docker attach maximize

docker-clean:
	@echo "Removing Docker containers, images, and volumes..."
	docker-compose down -v
	docker rmi maximize:latest 2>/dev/null || true
	@echo "Docker cleanup complete"

docker-shell:
	@echo "Opening shell in container..."
	docker exec -it maximize sh

# Show help
help:
	@echo "Maximize - Build System"
	@echo ""
	@echo "Available targets:"
	@echo ""
	@echo "Build & Run:"
	@echo "  make              - Build release version (default)"
	@echo "  make build        - Build debug version"
	@echo "  make release      - Build release version (optimized)"
	@echo "  make run          - Run debug version"
	@echo "  make run-release  - Run release version"
	@echo "  make run-debug    - Run with debug logging"
	@echo ""
	@echo "Docker:"
	@echo "  make docker         - Build and start Docker container"
	@echo "  make docker-build   - Build Docker image"
	@echo "  make docker-up      - Start Docker container"
	@echo "  make docker-down    - Stop Docker container"
	@echo "  make docker-logs    - View Docker logs"
	@echo "  make docker-attach  - Attach to container for auth"
	@echo "  make docker-clean   - Remove Docker containers/images"
	@echo "  make docker-shell   - Open shell in container"
	@echo ""
	@echo "Development:"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make test         - Run tests"
	@echo "  make check        - Check code without building"
	@echo "  make fmt          - Format code"
	@echo "  make lint         - Lint code with clippy"
	@echo ""
	@echo "Installation:"
	@echo "  make install      - Install binary to /usr/local/bin"
	@echo "  make uninstall    - Remove binary from /usr/local/bin"
	@echo ""
	@echo "  make help         - Show this help message"
