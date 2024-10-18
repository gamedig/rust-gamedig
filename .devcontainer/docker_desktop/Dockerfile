FROM mcr.microsoft.com/devcontainers/rust:1

ENV CROSS_CONTAINER_IN_CONTAINER=true

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive

RUN chown -R vscode /usr/local/cargo

RUN rustup toolchain install nightly

RUN cargo install cross