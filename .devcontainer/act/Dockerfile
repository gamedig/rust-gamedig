FROM mcr.microsoft.com/devcontainers/rust:1

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive

RUN chown -R vscode /usr/local/cargo

RUN rustup update
RUN rustup toolchain install nightly

RUN cargo install cargo-bom
RUN cargo install cargo-msrv
RUN cargo install cargo-audit
RUN cargo install cargo-expand



