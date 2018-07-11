FROM evernym/libsovtoken:0.2.0-xenial-base
# TODO LABEL maintainer="Name <email-address>"

ARG INDY_NODE_VERSION
ARG EVERNYM_REPO_IP
ARG POOL_IP

USER root

# python env necessary for indy-node
RUN apt-get update && apt-get install -y \
	    python3.5 \
	    python3-pip \
	    python-setuptools \
    && pip3 install -U \
	    setuptools \
        'pip<10.0.0' \
        setuptools \
    && rm -rf /var/lib/apt/lists/*

# indy-node along with supervisor
ENV INDY_NODE_VERSION ${INDY_NODE_VERSION:-1.4.496}
RUN echo "deb https://repo.sovrin.org/deb xenial master" >> /etc/apt/sources.list \
    && apt-get update && apt-get install -y \
	    supervisor \
        indy-node=${INDY_NODE_VERSION} \
    && rm -rf /var/lib/apt/lists/*
COPY supervisord.conf /etc/supervisord.conf

# config indy pool
ENV POOL_IP ${POOL_IP:-127.0.0.1}
USER indy
RUN awk '{if (index($1, "NETWORK_NAME") != 0) {print("NETWORK_NAME = \"sandbox\"")} else print($0)}' /etc/indy/indy_config.py> /tmp/indy_config.py \
    && mv /tmp/indy_config.py /etc/indy/indy_config.py \
    && generate_indy_pool_transactions --nodes 4 --clients 5 --nodeNum 1 2 3 4 --ips="$POOL_IP,$POOL_IP,$POOL_IP,$POOL_IP" \
    && chmod -R a+rw /var/lib/indy /var/log/indy /etc/indy

USER root

# sovtoken and sovtokenfees
ENV EVERNYM_REPO_IP ${EVERNYM_REPO_IP:-10.2.3.179}
# TODO workaround to copy optional files *.deb
# fix that once that issue is resovled
# https://github.com/moby/moby/issues/26332
COPY sovtoken*.deb libsovtoken-ci-entrypoint.sh /tmp/sovtoken-debs/
COPY Evernym_Root_CA.crt /usr/local/share/ca-certificates/Evernym_Root_CA.crt
RUN if [ -f /tmp/sovtoken-debs/sovtoken_*.deb -a -f /tmp/sovtoken-debs/sovtokenfees_*.deb ]; then \
        apt-get update \
        && dpkg -i /tmp/sovtoken-debs/*.deb \
        && apt-get install -f; \
    else \
        echo "${EVERNYM_REPO_IP} repo.corp.evernym.com" >> /etc/hosts \
        && update-ca-certificates \
        && curl https://repo.corp.evernym.com/repo.corp.evenym.com-sig.key | apt-key add - \
        && echo "deb https://repo.corp.evernym.com/deb evernym-agency-dev-ubuntu main" >> /etc/apt/sources.list \
        && apt-get update && apt-get install -y \
            sovtoken \
            sovtokenfees \
        && rm -rf /var/lib/apt/lists/*; \
    fi

USER root

COPY libsovtoken-ci-entrypoint.sh /usr/local/bin/
ENTRYPOINT ["libsovtoken-ci-entrypoint.sh"]

ENV LIBSOVTOKEN_CI_ENV_VERSION=0.13.0
