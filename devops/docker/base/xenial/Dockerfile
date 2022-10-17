FROM sovrin/dockerbase:rust-xenial-0.13.0
# TODO LABEL maintainer="Name <email-address>"

ARG u_id=1000
ARG u_name=user
# TODO
ARG INDY_SDK_VERSION

ENV LIBINDY_DIR=/usr/lib
ENV LIBSODIUM_LIB_DIR=/usr/lib
ENV LIBSODIUM_INC_DIR=/usr/include

# install libsodium from the sources
ENV LIBSODIUM_VERSION=1.0.16
RUN cd /tmp \
    && curl https://download.libsodium.org/libsodium/releases/old/libsodium-${LIBSODIUM_VERSION}.tar.gz | tar -xz \
    && cd /tmp/libsodium-${LIBSODIUM_VERSION} \
    && ./configure --prefix=/usr/local/ && make && make install \
    && ldconfig \
    && rm -rf /tmp/libsodium-${LIBSODIUM_VERSION}
# need for libsodium to be reachable via pkg-config (sodiumoxide uses it)
ENV PKG_CONFIG_PATH /usr/local/lib/pkgconfig:$PKG_CONFIG_PATH # TODO ??? is it really needed

ENV LIBINDY_VERSION=1.16.0-bionic
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88 \
    && echo "deb https://repo.sovrin.org/sdk/deb xenial stable" >> /etc/apt/sources.list \
    && echo "deb https://repo.sovrin.org/sdk/deb bionic stable" >> /etc/apt/sources.list \
    && echo "deb https://repo.sovrin.org/deb xenial master" >> /etc/apt/sources.list \
    && echo "deb http://archive.ubuntu.com/ubuntu bionic main" >> /etc/apt/sources.list \
    && echo "deb http://archive.ubuntu.com/ubuntu xenial main" >> /etc/apt/sources.list \
    && apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    libssl1.1 \
    libsodium23 \
    libindy=${LIBINDY_VERSION} \
    python3-ujson=1.33-1build1 \
    python3-pygments=2.2.0 \
    python3-leveldb \
    python3-six=1.11.0 \
    python3-msgpack=0.4.6-1build1 \
    python3-dateutil=2.6.1 \
    python3-rocksdb=0.6.9 \
    python3-setuptools=38.5.2 \
    python3-orderedset=2.0 \
    python3-psutil=5.4.3 \
    python3-pympler=0.5 \
    && rm -rf /var/lib/apt/lists/*


RUN if [ "$u_id" != "0" ]; then \
    useradd -ms /bin/bash -u $u_id $u_name; \
    fi

ENV TEST_USER_UID=$u_id

# fill cargo cache to speed up docker containers
COPY Cargo.toml /tmp/libsovtoken/
RUN chown -R ${u_id}:${u_id} /tmp/libsovtoken/
USER $u_id
RUN cd /tmp/libsovtoken \
    && cargo update \
    && rm -rf /tmp/libsovtoken/

# TODO CMD ENTRYPOINT ...


ENV LIBSOVTOKEN_BASE_ENV_VERSION=0.43.0
