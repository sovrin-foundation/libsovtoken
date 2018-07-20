FROM ubuntu:xenial
LABEL maintainer="Michael Lodder <redmike7@gmail.com>"

ARG indy_install

ENV PATH /home/token_user/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
COPY ${indy_install} /tmp/indy_install.sh

RUN apt-get -qq update -y && apt-get -qq install -y sudo zip unzip cmake autoconf libtool curl wget python3 pkg-config libssl-dev libzmq3-dev libsqlite3-dev 2>&1 > /dev/null \
    && bash /tmp/indy_install.sh \
    && useradd -m -d /home/token_user -s /bin/bash -p $(openssl passwd -1 "token") token_user \
    && usermod -aG sudo token_user \
    && cd /tmp \
    && curl https://download.libsodium.org/libsodium/releases/libsodium-1.0.14.tar.gz | tar -xz \
    && cd libsodium-1.0.14 \
    && ./autogen.sh \
    && ./configure \
    && make \
    && make install \
    && cd .. \
    && rm -rf libsodium-1.0.14

USER token_user
WORKDIR /home/token_user
COPY --chown=token_user:token_user playground/ /home/token_user/playground/
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && cargo install cargo-deb \
    && cd playground \
    && cargo build \
    ; cd .. \
    && rm -rf playground \
    && echo "libsovtoken configured successful"
