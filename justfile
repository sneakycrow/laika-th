set dotenv-load := true

# Lists out all available commands
default:
    just --list

# Builds the Rust API using Cargo
build-api:
    cargo build --release

# Builds the frontend web distributables using npm
build-web:
    npm run build --workspace=web

# Shorthand for building frontend and api
build: build-web build-api

# Runs the api using Cargo
dev-api:
    cargo run

# Runs the frontend using node/vite
dev-web:
    npm run dev --workspace=web

# Initializes the project
init:
    @sh ./scripts/init.sh
    @just install

# Shorthand for install all dependencies
install: install-web install-api

# Installs the web depdendencies
install-web:
    npm i

# Installs the api depdendencies
install-api:
    cargo check

# Cleans the configured data directory
clean-data:
    rm -rf $STORAGE_PATH/*
