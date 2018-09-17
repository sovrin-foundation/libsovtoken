FROM evernym/libsovtoken:base-xenial-0.9.0
# TODO LABEL maintainer="Name <email-address>"

ARG LIBINDY_CRYPTO_VERSION
ARG PYTHON3_INDY_CRYPTO_VERSION
ARG INDY_PLENUM_VERSION
ARG INDY_ANONCREDS_VERSION
ARG INDY_NODE_VERSION

ARG EVERNYM_REPO_IP
ARG POOL_IP

USER root

# python env necessary for indy-node
RUN apt-get update && apt-get install -y --no-install-recommends \
	    supervisor \
	    python3.5 \
	    python3-pip \
	    python-setuptools \
    && pip3 install -U \
	    setuptools \
        'pip<10.0.0' \
        setuptools \
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
ENV LIBINDY_CRYPTO_VERSION ${LIBINDY_CRYPTO_VERSION:-0.4.3}
ENV PYTHON3_INDY_CRYPTO_VERSION ${PYTHON3_INDY_CRYPTO_VERSION:-0.4.3}
ENV INDY_PLENUM_VERSION ${INDY_PLENUM_VERSION:-1.6.51}
ENV INDY_ANONCREDS_VERSION ${INDY_ANONCREDS_VERSION:-1.0.11}
ENV INDY_NODE_VERSION ${INDY_NODE_VERSION:-1.6.73}
RUN echo "deb https://repo.sovrin.org/sdk/deb xenial stable" >> /etc/apt/sources.list
RUN echo "deb https://repo.sovrin.org/deb xenial stable" >> /etc/apt/sources.list \
    && apt-get update && apt-get install -y --no-install-recommends \
        libindy-crypto=${LIBINDY_CRYPTO_VERSION} \
        python3-indy-crypto=${PYTHON3_INDY_CRYPTO_VERSION} \
        indy-plenum=${INDY_PLENUM_VERSION} \
        indy-anoncreds=${INDY_ANONCREDS_VERSION} \
        indy-node=${INDY_NODE_VERSION} \
    && rm -rf /var/lib/apt/lists/*
COPY supervisord.conf /etc/supervisord.conf


# config indy pool
ENV POOL_IP ${POOL_IP:-127.0.0.1}
USER indy
RUN awk '{if (index($1, "NETWORK_NAME") != 0) {print("NETWORK_NAME = \"sandbox\"")} else print($0)}' /etc/indy/indy_config.py> /tmp/indy_config.py \
    && mv /tmp/indy_config.py /etc/indy/indy_config.py \
    && chmod -R 777 /var/lib/indy /var/log/indy /etc/indy

USER root

# sovtoken and sovtokenfees
# bump this env var if you need to avoid caching for apt-based installation
ENV SOVTOKEN_PLUGINS_APT_UPDATE 1
ENV EVERNYM_REPO_IP ${EVERNYM_REPO_IP:-10.2.3.179}
# TODO workaround to copy optional files *.deb
# fix that once that issue is resovled
# https://github.com/moby/moby/issues/26332
COPY sovtoken*.deb supervisord.conf /tmp/sovtoken-debs/
RUN if [ -f /tmp/sovtoken-debs/sovtoken_*.deb ] && [ -f /tmp/sovtoken-debs/sovtokenfees_*.deb ]; then \
        apt-get update \
        && dpkg -i /tmp/sovtoken-debs/*.deb \
        && apt-get install -f -y --no-install-recommends; \
    else \
        echo "${EVERNYM_REPO_IP} repo.corp.evernym.com" >> /etc/hosts \
        && mkdir -p /usr/local/share/ca-certificates \
        && curl -k https://repo.corp.evernym.com/ca.crt | tee /usr/local/share/ca-certificates/Evernym_Root_CA.crt \
        && update-ca-certificates \
        && curl https://repo.corp.evernym.com/repo.corp.evenym.com-sig.key | apt-key add - \
        && echo 'deb https://repo.corp.evernym.com/deb evernym-agency-dev-ubuntu main' | tee /etc/apt/sources.list.d/agency-dev_repo.corp.evernym.com.list \
        && apt-get update && apt-get install -y --no-install-recommends \
            sovtoken \
            sovtokenfees \
        && rm -rf /var/lib/apt/lists/*; \
    fi


COPY libsovtoken-ci-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/libsovtoken-ci-entrypoint.sh
ENTRYPOINT ["libsovtoken-ci-entrypoint.sh"]

ENV LIBSOVTOKEN_CI_ENV_VERSION=0.42.0
