# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.205.2/containers/ubuntu/.devcontainer/base.Dockerfile

# [Choice] Ubuntu version (use hirsuite or bionic on local arm64/Apple Silicon): hirsute, focal, bionic
ARG VARIANT="focal"
FROM mcr.microsoft.com/vscode/devcontainers/base:${VARIANT}

RUN apt-get update -y && apt-get install -y \
    # common stuff
    git \
    wget \
    gnupg \
    apt-transport-https \
    ca-certificates \
    apt-utils \
    curl \
    jq

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys CE7709D068DB5E88

RUN echo "deb https://repo.sovrin.org/sdk/deb bionic master" >> /etc/apt/sources.list

RUN apt-get update -y && apt-get install -y \
    libssl-dev \
    pkg-config \
    libindy=1.15.0~1625-bionic

# For some reason this file gets added automatically and contains
# deb https://repo.sovrin.org/sdk/deb focal master
# which does not exist and causes issues when packages are added afterward.
RUN rm /etc/apt/sources.list.d/sovrin.list