.PHONY: help build run stop clean test migrate fmt lint

help:
	@echo "Notes API - Available Commands"
	@echo "=============================="
	@echo ""
	@echo "Development:"
	@echo "  make build      - Build the project"
	@echo "  make run        - Run the development server"
	@echo "  make stop       - Stop the development server"
	@echo "  make test       - Run tests"
	@echo "  make fmt        - Format code"
	@echo "  make lint       - Check code with clippy"
	@echo ""
	@echo "Database:"
	@echo "  make migrate    - Run database migrations"
	@echo "  make db-create  - Create database"
	@echo "  make db-drop    - Drop database"
	@echo "  make db-reset   - Reset database (drop and create)"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-up      - Start Docker services"
	@echo "  make docker-down    - Stop Docker services"
	@echo "  make docker-logs    - View Docker logs"
	@echo "  make docker-build   - Build Docker image"
	@echo "  make docker-shell   - Shell into API container"
	@echo ""

# Development Commands
build:
	cargo build

run:
	cargo run

stop:
	@pkill -f "target/debug/notes-api" || true

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

# Database Commands
migrate:
	sqlx migrate run

db-create:
	sqlx database create

db-drop:
	sqlx database drop -y

db-reset: db-drop db-create migrate

# Docker Commands
docker-up:
	docker compose up -d

docker-down:
	docker compose down

docker-logs:
	docker compose logs -f

docker-build:
	docker compose build --no-cache

docker-shell:
	docker compose exec api bash

docker-db-shell:
	docker compose exec postgres psql -U postgres -d notes_db

# Combined Commands
dev-setup: db-create migrate
	@echo "✅ Development environment setup complete"

dev-clean: db-drop stop
	@echo "✅ Development environment cleaned"

dev-reset: db-reset run
	@echo "✅ Development environment reset"
