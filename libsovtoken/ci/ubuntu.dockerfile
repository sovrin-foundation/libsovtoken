FROM ubuntu:16.04
LABEL maintainer="Michael Lodder <redmike7@gmail.com>"

ARG apt_cmd
ARG indy_pkg

ENV SODIUM_LIB_DIR /home/token_user/libsodium/lib
ENV SODIUM_INCLUDE_DIR /home/token_user/libsodium/include
ENV SODIUM_STATIC=1
ENV PATH /home/token_user/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN ${apt_cmd} apt-get -qq update -y && apt-get -qq install -y sudo zip unzip cmake autoconf libtool curl wget python3 pkg-config libssl-dev libzmq3-dev libsqlite3-dev ${indy_pkg} 2>&1 > /dev/null
RUN useradd -m -d /home/token_user -s /bin/bash -p $(openssl passwd -1 "token") token_user
RUN usermod -aG sudo token_user

USER token_user
WORKDIR /home/token_user

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN wget -q https://download.libsodium.org/libsodium/releases/libsodium-1.0.14.tar.gz
RUN tar xf /home/token_user/libsodium-1.0.14.tar.gz
WORKDIR /home/token_user/libsodium-1.0.14
RUN ./autogen.sh
RUN ./configure --prefix=/home/token_user/libsodium
RUN make
RUN make install

WORKDIR /home/token_user
RUN rm -f libsodium-1.0.14.tar.gz
RUN rm -rf libsodium-1.0.14
RUN echo "libsovtoken configured successful"
