# GameDig Dev Containers

This setup guide is for configuring a unified, containerized environment tailored for both **Docker Desktop** (local development) and **GitHub Codespaces** (cloud development). The configurations use Visual Studio Code’s Dev Containers to streamline environment setup and maintain consistency across different operating systems.

---

## What Are Dev Containers?

Dev Containers allow you to run a development environment inside a Docker container, providing an isolated workspace with pre-installed dependencies, tools, and configurations. This means:

- **Reduced Setup Time**: All tools, dependencies, and configurations are predefined.
- **Platform Consistency**: Every developer gets an identical environment, avoiding "works on my machine" issues.
- **Flexibility**: Supports running on Windows, macOS, and Linux systems.

For a general introduction, see [Microsoft’s Dev Containers Overview](https://code.visualstudio.com/docs/devcontainers/containers).

## Dev Container Images

The GameDig project includes two Dev Container configurations optimized for Docker Desktop and GitHub Codespaces:

- **Docker Desktop**: Designed for local development.
- **GitHub Codespaces**: Optimized for remote, cloud-based development on GitHub.

### Key Features in Both Images:

- **Rust Development Tools**: Rust is pre-installed with nightly toolchain.
- **VS Code Extensions**: A comprehensive set of extensions for Rust development, code visualization, and Docker management.
- **Utility Tools**:
  - **`just` command runner** for task automation
  - **`act`** for running GitHub Actions locally
  - **Docker** integration for Docker-in-Docker or Docker-outside-of-Docker, depending on the environment.

---

## Setup Instructions

### Prerequisites

Ensure you have the following tools installed:

- **Visual Studio Code**: [Download here](https://code.visualstudio.com/).
- **Dev Containers Extension**: Install [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers).
- **Docker**: Refer to the OS-specific steps on the [Docker website](https://docs.docker.com/get-docker).

---

> [!NOTE]
> Work in progress. to be continued...
