# Show this help
help:
    @just --list

# Conduct a full release build
build:
    @# This causes cargo to have to rebuild the binary,
    @# even if no Rust code has changed either.
    @cargo build --release

# Conduct unit tests
test:
    @cargo test -- --nocapture

# Conduct DB-related unit tests
testdb:
    @# Be sure run the following command before conducting this:
    @# $ docker compose -f ./devops/local/docker-compose.yaml up
    @cargo test -- --nocapture --ignored

# Build Kotosiro into a docker image for local use
package:
    DOCKER_BUILDKIT=1 docker build . -t kotosiro:local -f devops/docker/Dockerfile