set dotenv-load := true

default:
    just --list

build-api:
    cargo build --release

build-web:
    npm run build --workspace=web

build: build-web build-api

dev-api:
    cargo run

dev-web:
    npm run dev --workspace=web

init:
    @sh ./scripts/init.sh
    @just install

install: install-web install-api

install-web:
    npm i

install-api:
    cargo check

clean-data:
    rm -rf $STORAGE_PATH/*
