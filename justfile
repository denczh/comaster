default: build

build:
    cargo build --workspace

test:
    cargo test --workspace

fmt:
    cargo fmt --all

clippy:
    cargo clippy --workspace -- -D warnings

run-comaster:
    cargo run -p comaster

run-mcp name:
    cargo run -p {{name}}-mcp
