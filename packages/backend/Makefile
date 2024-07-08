# Makefile: rapinsel

dev:
	cargo watch -q -c -w src/ -x run

build:
	cargo build

# Docker
dup:
	docker-compose up -d
ddown:
	docker-compose down

run_migrations:
	sqlx migrate run