FROM cruizba/ubuntu-dind

ARG DEBIAN_FRONTEND=noninteractive
ENV LIBINDY_DIR=/usr/lib
RUN apt-get update -y && apt-get install -y \
    # common stuff
    git \
    wget \
    gnupg \
    apt-transport-https \
    ca-certificates \
    apt-utils \
    software-properties-common\
    rustc\
    curl

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