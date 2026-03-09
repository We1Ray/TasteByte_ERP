# ============================================================
# TasteByte ERP — Root Makefile
# ============================================================

.PHONY: dev build test lint docker-up docker-down docker-logs docker-ps docker-clean db-backup db-restore db-reset db-shell clean help

ROOT_DIR := $(shell pwd)
BACKEND_DIR := $(ROOT_DIR)/backend
WEB_DIR := $(ROOT_DIR)/web

# ── Default target ───────────────────────────────────────────
help:
	@echo "TasteByte ERP — Available targets:"
	@echo ""
	@echo "  make dev          Start all services locally (backend + web)"
	@echo "  make dev-backend  Start backend only"
	@echo "  make dev-web      Start web frontend only"
	@echo "  make build        Build all (backend + web)"
	@echo "  make test         Run all tests (backend + web)"
	@echo "  make lint         Run linters (clippy + eslint)"
	@echo "  make docker-up    Build and start Docker containers"
	@echo "  make docker-down  Stop and remove Docker containers"
	@echo "  make docker-logs  View Docker Compose logs"
	@echo "  make docker-clean Remove Docker volumes and images"
	@echo "  make db-backup    Run database backup script"
	@echo "  make db-restore   Restore database from backup (BACKUP=path/to/file)"
	@echo "  make db-reset     Drop and recreate database"
	@echo "  make db-shell     Open psql console"
	@echo "  make clean        Clean build artifacts"
	@echo "  make help         Show this help message"

# ── Development ──────────────────────────────────────────────
dev:
	@echo "Starting TasteByte ERP in development mode..."
	@trap 'kill 0' EXIT; \
	(cd $(BACKEND_DIR) && cargo run) & \
	(cd $(WEB_DIR) && pnpm dev) & \
	wait

dev-backend:
	cd $(BACKEND_DIR) && cargo run

dev-web:
	cd $(WEB_DIR) && pnpm dev

# ── Build ────────────────────────────────────────────────────
build: build-backend build-web

build-backend:
	@echo "Building backend..."
	cd $(BACKEND_DIR) && cargo build --release

build-web:
	@echo "Building web frontend..."
	cd $(WEB_DIR) && pnpm install --frozen-lockfile && pnpm build

# ── Test ─────────────────────────────────────────────────────
test: test-backend test-web

test-backend:
	@echo "Running backend tests..."
	cd $(BACKEND_DIR) && cargo test

test-web:
	@echo "Running web tests..."
	cd $(WEB_DIR) && pnpm test || true

# ── Lint ─────────────────────────────────────────────────────
lint: lint-backend lint-web

lint-backend:
	@echo "Linting backend (clippy)..."
	cd $(BACKEND_DIR) && cargo clippy -- -D warnings

lint-web:
	@echo "Linting web (eslint)..."
	cd $(WEB_DIR) && pnpm lint

# ── Docker ───────────────────────────────────────────────────
docker-up:
	@echo "Starting Docker containers..."
	docker compose up --build -d

docker-down:
	@echo "Stopping Docker containers..."
	docker compose down

docker-logs:
	docker compose logs -f

docker-ps:
	docker compose ps

docker-clean:
	@echo "Removing Docker containers, volumes, and images..."
	docker compose down -v --rmi local

# ── Database ─────────────────────────────────────────────────
db-backup:
	@echo "Running database backup..."
	$(ROOT_DIR)/scripts/backup.sh

db-restore:
ifndef BACKUP
	@echo "Error: BACKUP variable not set."
	@echo "Usage: make db-restore BACKUP=path/to/backup.sql.gz"
	@exit 1
endif
	$(ROOT_DIR)/scripts/restore.sh $(BACKUP)

db-reset:
	@echo "Dropping and recreating TastyByte database..."
	PGPASSWORD=postgres dropdb -h localhost -p 5432 -U postgres --if-exists TastyByte
	PGPASSWORD=postgres createdb -h localhost -p 5432 -U postgres TastyByte
	@echo "Database reset complete. Run 'make dev-backend' to apply migrations."

db-shell:
	PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -d TastyByte

# ── Clean ────────────────────────────────────────────────────
clean:
	@echo "Cleaning build artifacts..."
	cd $(BACKEND_DIR) && cargo clean
	rm -rf $(WEB_DIR)/.next $(WEB_DIR)/node_modules
