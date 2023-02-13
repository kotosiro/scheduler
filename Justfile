# Show this help
help:
    @just --list

# Conduct a full release build
build:
    @# This causes cargo to have to rebuild the binary,
    @# even if no Rust code has changed either.
    @cargo build --release