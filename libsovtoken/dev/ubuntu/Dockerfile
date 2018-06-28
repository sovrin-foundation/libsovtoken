FROM ubuntu:16.04
LABEL maintainer="Michael Lodder <redmike7@gmail.com>"

ARG indy_install

ENV PATH /home/token_user/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN apt-get -qq update -y && apt-get -qq install -y sudo zip unzip cmake autoconf libtool curl wget python3 pkg-config libssl-dev libzmq3-dev libsqlite3-dev 2>&1 > /dev/null
COPY ${indy_install} /tmp/indy_install.sh
RUN bash /tmp/indy_install.sh
RUN useradd -m -d /home/token_user -s /bin/bash -p $(openssl passwd -1 "token") token_user
RUN usermod -aG sudo token_user

WORKDIR /tmp
RUN wget -q https://download.libsodium.org/libsodium/releases/libsodium-1.0.14.tar.gz
RUN tar xf /tmp/libsodium-1.0.14.tar.gz
RUN rm -f libsodium-1.0.14.tar.gz
WORKDIR /tmp/libsodium-1.0.14
RUN ./autogen.sh
RUN ./configure
RUN make
RUN make install

USER token_user
WORKDIR /home/token_user
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN cargo install cargo-deb

RUN echo "libsovtoken configured successful"
