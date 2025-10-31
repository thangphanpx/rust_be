# Makefile for Rust Backend Project

.PHONY: help build run test clean docker-build docker-run docker-stop setup check format

help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

setup: ## Setup development environment
	@echo "Setting up development environment..."
	@cp .env.example .env
	@echo "Please edit .env file with your database configuration"

check: ## Check code without building
	cargo check

build: ## Build the project
	cargo build

build-release: ## Build the project in release mode
	cargo build --release

run: ## Run the application
	cargo run

test: ## Run tests
	cargo test

format: ## Format code
	cargo fmt

lint: ## Run clippy linter
	cargo clippy -- -D warnings

clean: ## Clean build artifacts
	cargo clean

# Docker commands
docker-build: ## Build Docker image
	docker build -t rust_be .

docker-run: ## Run application with Docker Compose
	docker-compose up -d

docker-stop: ## Stop Docker containers
	docker-compose down

docker-logs: ## View Docker logs
	docker-compose logs -f app

docker-clean: ## Clean Docker containers and volumes
	docker-compose down -v
	docker system prune -f

# Database commands
db-setup: ## Setup database with Docker
	docker run --name postgres_rust_be -e POSTGRES_DB=rust_be_db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres:15-alpine

db-stop: ## Stop database container
	docker stop postgres_rust_be
	docker rm postgres_rust_be

# Development commands
dev: ## Run in development mode with auto-reload (requires cargo-watch)
	cargo watch -x run

install-tools: ## Install development tools
	cargo install cargo-watch
	cargo install sea-orm-cli

# API testing
test-health: ## Test health endpoint
	curl http://localhost:3000/api/v1/health

test-api: ## Run basic API tests
	@echo "Testing Health endpoint..."
	@curl -s http://localhost:3000/api/v1/health | jq .
	@echo "\nTesting Users endpoint..."
	@curl -s http://localhost:3000/api/v1/users | jq .

# Swagger
open-swagger: ## Open Swagger UI in browser (Windows)
	start http://localhost:3000/swagger-ui/