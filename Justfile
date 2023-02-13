# Show this help
help:
    @just --list

# Conduct a full release build
build:
    @# This causes cargo to have to rebuild the binary,
    @# even if no Rust code has changed either.
    @cargo build --release

# Build Kotosiro into a docker image for local use
package:
    DOCKER_BUILDKIT=1 docker build . -t kotosiro:local -f devops/docker/Dockerfile