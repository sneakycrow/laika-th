set dotenv-load := true

build-api:
    cargo build --release

build-web:
    npm run build --workspace=web

build: build-web build-api

dev-api:
    cargo run

init:
    @sh ./scripts/init.sh
