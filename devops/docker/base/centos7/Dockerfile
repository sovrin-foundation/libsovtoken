FROM sovrin/dockerbase:rust-centos7-0.5.0
# TODO LABEL maintainer="Name <email-address>"

ARG u_id=1000
ARG u_name=user
ARG INDY_SDK_VERSION

RUN yum install -y \
# zeromq is available in EPEL
        epel-release \
    && yum install -y \
        sqlite-devel \
        openssl-devel \
        ncurses-devel \
        zeromq-devel \
        libsodium-devel \
    && yum clean all

# install recent libsodium version from the sources
ENV LIBSODIUM_VERSION=1.0.14
RUN cd /tmp && \
    curl https://download.libsodium.org/libsodium/releases/libsodium-${LIBSODIUM_VERSION}.tar.gz | tar -xz && \
    cd /tmp/libsodium-${LIBSODIUM_VERSION} && \
    ./configure --prefix=/usr/local/ && make && make install && \
    rm -rf /tmp/libsodium-${LIBSODIUM_VERSION}
# need for libsodium to be reachable via pkg-config
ENV PKG_CONFIG_PATH /usr/local/lib/pkgconfig:$PKG_CONFIG_PATH

ENV INDY_SDK_VERSION=${INDY_SDK_VERSION:-1.4.0}
RUN cd /tmp \
    && curl -L https://github.com/hyperledger/indy-sdk/archive/v${INDY_SDK_VERSION}.zip -o indy-sdk.zip \
    && unzip indy-sdk.zip \
    && cd indy-sdk-${INDY_SDK_VERSION}/libindy \
    && echo "WARN: cargo build progress for registry update is not visible, could be quite long..." \
    && cargo build --release \
    && cp target/release/libindy.so /usr/local/lib \
    && rm -rf /tmp/indy-sdk* \
    && rm -rf /usr/local/cargo/registry

RUN useradd -ms /bin/bash -u $u_id $u_name
USER $u_name

# TODO CMD ENTRYPOINT ...
WORKDIR /home/$u_name

ENV LIBSOVTOKEN_BASE_ENV_VERSION=0.2.0
