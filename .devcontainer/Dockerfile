FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    sudo git curl build-essential pkg-config libssl-dev ca-certificates bash-completion vim \
    && rm -rf /var/lib/apt/lists/*

USER ubuntu
WORKDIR /home/ubuntu

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && . "$HOME/.cargo/env" \
    && rustup component add rustfmt clippy rust-src

WORKDIR /workspaces/skju
