set dotenv-load := true

build:
    cargo build --release

dev:
    cargo run

init:
    @sh ./scripts/init.sh
