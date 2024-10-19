FROM mcr.microsoft.com/devcontainers/rust:1

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive

RUN chown -R vscode /usr/local/cargo

RUN rustup toolchain install nightly