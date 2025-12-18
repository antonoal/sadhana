# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.209.6/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bookworm"
FROM mcr.microsoft.com/devcontainers/rust:1-${VARIANT}

# ---- system deps ----
USER root

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        libpq-dev \
        pkg-config \
        curl \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# ---- Node.js (for Playwright only) ----
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g npm \
    && node --version \
    && npm --version

# ---- Playwright OS dependencies ----
RUN npx playwright install-deps

USER vscode

# ---- Rust / WASM tooling ----
RUN rustup target add wasm32-unknown-unknown

RUN cargo install cargo-binstall \
    && cargo binstall -y trunk wasm-bindgen-cli \
    && cargo install diesel_cli --no-default-features --features postgres \
    && cargo install leptosfmt