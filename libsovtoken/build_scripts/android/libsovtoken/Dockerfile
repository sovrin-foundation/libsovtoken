FROM ubuntu:16.04
LABEL maintainer="Michael Lodder <redmike7@gmail.com>"

ENV PATH /root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN apt-get -qq update \
    && apt-get -qq install -y wget curl libtool unzip zip python3 pkg-config git 2>&1 > /dev/null \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && /root/.cargo/bin/rustup target add arm-linux-androideabi armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android \
    && echo "libsovtoken android image successful"
