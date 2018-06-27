FROM evernym/rust:0.3.0-xenial
# TODO LABEL maintainer="Name <email-address>"

ARG u_id=1000
ARG u_name=user

ENV LIBINDY_DIR=/usr/lib

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88 \
    && echo "deb https://repo.sovrin.org/sdk/deb xenial master" >> /etc/apt/sources.list \
    && apt-get update && apt-get install -y --no-install-recommends \
        libindy \
    && rm -rf /var/lib/apt/lists/*

RUN apt-get install -y cmake autoconf libtool

WORKDIR /tmp
RUN wget -q https://download.libsodium.org/libsodium/releases/libsodium-1.0.14.tar.gz
RUN tar xf /tmp/libsodium-1.0.14.tar.gz
RUN rm -f libsodium-1.0.14.tar.gz
WORKDIR /tmp/libsodium-1.0.14
RUN ./autogen.sh
RUN ./configure
RUN make
RUN make install

RUN useradd -ms /bin/bash -u $u_id $u_name
USER $u_name

# TODO CMD ENTRYPOINT ...
WORKDIR /home/$u_name

ENV CI_ENV_VERSION=0.5.0
