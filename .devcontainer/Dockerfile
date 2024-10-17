FROM mcr.microsoft.com/devcontainers/rust:1

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
     && chown -R vscode /usr/local/cargo

COPY .env /.env
