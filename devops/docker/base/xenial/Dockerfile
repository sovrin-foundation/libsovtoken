FROM evernym/rust:0.4.0-xenial
# TODO LABEL maintainer="Name <email-address>"

ARG u_id=1000
ARG u_name=user
# TODO
ARG INDY_SDK_VERSION

ENV LIBINDY_DIR=/usr/lib
ENV LIBSODIUM_LIB_DIR=/usr/lib
ENV LIBSODIUM_INC_DIR=/usr/include

# install libsodium from the sources
ENV LIBSODIUM_VERSION=1.0.14
RUN cd /tmp \
    && curl https://download.libsodium.org/libsodium/releases/libsodium-${LIBSODIUM_VERSION}.tar.gz | tar -xz \
    && cd /tmp/libsodium-${LIBSODIUM_VERSION} \
    && ./configure --prefix=/usr/local/ && make && make install \
    && ldconfig \
    && rm -rf /tmp/libsodium-${LIBSODIUM_VERSION}
# need for libsodium to be reachable via pkg-config (sodiumoxide uses it)
ENV PKG_CONFIG_PATH /usr/local/lib/pkgconfig:$PKG_CONFIG_PATH # TODO ??? is it really needed

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88 \
    && echo "deb https://repo.sovrin.org/sdk/deb xenial master" >> /etc/apt/sources.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends libssl-dev libindy \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash -u $u_id $u_name
ENV TEST_USER=$u_name
ENV TEST_USER_UID=$u_id

# fill cargo cache to speed up docker containers
COPY Cargo.toml /tmp/libsovtoken/
RUN chown -R ${u_name}:${u_name} /tmp/libsovtoken/
USER $u_name
RUN cd /tmp/libsovtoken \
    && cargo update \
    && rm -rf /tmp/libsovtoken/

# TODO CMD ENTRYPOINT ...

ENV LIBSOVTOKEN_BASE_ENV_VERSION=0.2.0