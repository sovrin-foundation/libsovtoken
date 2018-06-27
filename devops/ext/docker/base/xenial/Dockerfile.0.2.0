FROM ubuntu:xenial
# TODO LABEL maintainer="Name <email-address>"

# generally useful packages
RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        apt-transport-https \
        curl \
        wget \
        vim \
        git \
        procps \
        autoconf \
        automake \
        g++ \
        gcc \
        make \
        pkg-config \
        zip \
        unzip \
    && rm -rf /var/lib/apt/lists/*

# install fpm
ENV FPM_VERSION=1.9.3
RUN apt-get update && apt-get install -y --no-install-recommends \
        ruby \
        ruby-dev \
        rubygems \
    && gem install --no-ri --no-rdoc fpm -v $FPM_VERSION \
    && rm -rf /var/lib/apt/lists/*

# TODO CMD ENTRYPOINT ...

ENV BASE_ENV_VERSION=0.2.0
