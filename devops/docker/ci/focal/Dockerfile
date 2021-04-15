FROM sovrin/libsovtoken:base-focal-0.39.0
# TODO LABEL maintainer="Name <email-address>"

ARG LIBINDY_CRYPTO_VERSION
ARG PYTHON3_INDY_CRYPTO_VERSION
ARG INDY_PLENUM_VERSION
ARG INDY_NODE_VERSION

ARG SOVRIN_REPO_IP
ARG POOL_IP

USER root

# python env necessary for indy-node
RUN echo "deb http://security.ubuntu.com/ubuntu bionic main"  >> /etc/apt/sources.list \
    && apt-get update && apt-get install -y --no-install-recommends \
    supervisor \
    ursa \
    # Python
    python3-pip \
    python3-nacl \
    # rocksdb python wrapper
    rocksdb-tools \
    librocksdb5.17 \
    librocksdb-dev \
    libsnappy-dev \
    liblz4-dev \
    libbz2-dev \
    # lib ssl 
    libssl1.0.0 \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*
# indy-node along with dependencies
# note:
#  - indy-node in master/stable components of apt repo is published along with
#    all dependencies it needs and presented in the same repo (plenum, anoncreds, crypto...)
#  - no old versions are cleaned, thus to avoid problems with apt during install
#    it's necessary to specify versions explicitly for dependencies that:
#       - have explicitly defined versions in packages depends list
#       - are likely to be updated (e.g. packaged 3rd parties like python3-pyzmq
#         or python3-rocksdb are not specified here)
ENV LIBINDY_CRYPTO_VERSION ${LIBINDY_CRYPTO_VERSION:-0.4.5}
ENV PYTHON3_INDY_CRYPTO_VERSION ${PYTHON3_INDY_CRYPTO_VERSION:-0.4.5}
ENV TOKEN_VER  ${TOKEN_VER:-1.0.8~dev153}
ENV INDY_PLENUM_VERSION ${INDY_PLENUM_VERSION:-1.13.0~dev1033}
ENV INDY_NODE_VERSION ${INDY_NODE_VERSION:-1.13.0~dev1225}
RUN apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys F23C5A6CF475977595C89F51BA6932366A755776 \
    && echo "deb https://repo.sovrin.org/sdk/deb xenial master" >> /etc/apt/sources.list \
    && echo "deb https://repo.sovrin.org/deb xenial master" >> /etc/apt/sources.list \
    && echo "deb http://archive.ubuntu.com/ubuntu xenial universe" >> /etc/apt/sources.list \
    && echo "deb http://archive.ubuntu.com/ubuntu xenial main" >> /etc/apt/sources.list \
    && echo "deb http://archive.ubuntu.com/ubuntu bionic universe" >> /etc/apt/sources.list \
    && echo "deb http://archive.ubuntu.com/ubuntu bionic main" >> /etc/apt/sources.list \
    && echo "deb http://ppa.launchpad.net/deadsnakes/ppa/ubuntu bionic main" >> /etc/apt/sources.list \
    && apt-get update && apt-get install -y --no-install-recommends --allow-downgrades \
    libindy=1.15.0~1625-bionic \
    libsodium23 \
    python3-setuptools \
    python3-pyzmq=18.1.0 \
    indy-node=${INDY_NODE_VERSION} \
    indy-plenum=${INDY_PLENUM_VERSION} \
    sovtoken=${TOKEN_VER} \
    sovtokenfees=${TOKEN_VER} \
    python3-distro=1.3.0 \
    python3-jsonpickle=0.9.6  \
    python3-ujson=1.33-1build1 \
    python3-prompt-toolkit=0.57-1 \ 
    python3-pygments=2.2.0 \
    python3-rlp=0.5.1 \
    python3-sha3=0.2.1 \
    python3-setuptools=38.5.2 \
    python3-leveldb=0~svn68-2build4 \
    python3-base58=1.0.0 \
    python3-sortedcontainers=1.5.7 \
    python3-pip=9.0.1-2 \
    python3=3.5.1-3 \
    python-pip-whl=9.0.1-2 \
    python3-portalocker=0.5.7 \
    python3-libnacl=1.6.1 \
    python3-six=1.11.0 \
    python3-intervaltree=2.1.0 \
    python3-msgpack=0.4.6-1build1 \
    python3-dateutil=2.6.1 \
    python3-packaging=19.0 \
    python3-rocksdb=0.6.9 \
    python3-minimal=3.5.1-3 \
    libpython3-stdlib=3.5.1-3 \
    && rm -rf /var/lib/apt/lists/*
COPY supervisord.conf /etc/supervisord.conf

RUN pip3 install -U \
    setuptools \
    pipenv \
    python-ursa==0.1.1 \
    python3-indy==1.15.0-dev-1625 \
    'pip<10.0.0' 

# config indy pool
ENV POOL_IP ${POOL_IP:-127.0.0.1}
USER indy
RUN awk '{if (index($1, "NETWORK_NAME") != 0) {print("NETWORK_NAME = \"sandbox\"")} else print($0)}' /etc/indy/indy_config.py> /tmp/indy_config.py \
    && mv /tmp/indy_config.py /etc/indy/indy_config.py \
    && chmod -R 777 /var/lib/indy /var/log/indy /etc/indy

USER root

ENV SOVTOKEN_PLUGINS_APT_UPDATE 2

COPY libsovtoken-ci-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/libsovtoken-ci-entrypoint.sh
ENTRYPOINT ["libsovtoken-ci-entrypoint.sh"]

ENV LIBSOVTOKEN_CI_ENV_VERSION=0.732.0
