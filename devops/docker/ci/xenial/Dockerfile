FROM evernym/rust:0.3.1-xenial
# TODO LABEL maintainer="Name <email-address>"

ARG u_id=1000
ARG u_name=user

ENV LIBINDY_DIR=/usr/lib
ENV LIBSODIUM_LIB_DIR=/usr/lib
ENV LIBSODIUM_INCLUDE_DIR=/usr/include

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88 \
    && echo "deb https://repo.sovrin.org/sdk/deb xenial master" >> /etc/apt/sources.list \
    && apt-get update && apt-get install -y --no-install-recommends \
        libindy \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash -u $u_id $u_name
USER $u_name

# TODO CMD ENTRYPOINT ...
WORKDIR /home/$u_name

ENV CI_ENV_VERSION=0.5.1
