FROM evernym/libsovtoken:0.1.0-xenial-base
# TODO LABEL maintainer="Name <email-address>"

ARG indy_stream=master
ARG indy_plenum_ver=1.4.419
ARG indy_anoncreds_ver=1.0.32
ARG indy_node_ver=1.4.480
ARG python3_indy_crypto_ver=0.4.1
ARG indy_crypto_ver=0.4.0
ARG pool_ip=127.0.0.1

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

RUN echo "deb https://repo.sovrin.org/deb xenial $indy_stream" >> /etc/apt/sources.list \
    && apt-get update && apt-get install -y \
	    supervisor \
        indy-plenum=${indy_plenum_ver} \
        indy-anoncreds=${indy_anoncreds_ver} \
        indy-node=${indy_node_ver} \
        python3-indy-crypto=${python3_indy_crypto_ver} \
        libindy-crypto=${indy_crypto_ver} \
    && rm -rf /var/lib/apt/lists/*

COPY Evernym_Root_CA.crt /usr/local/share/ca-certificates/Evernym_Root_CA.crt
COPY repo.corp.evenym.com-sig.key /tmp/repo.corp.evenym.com-sig.key

#RUN update-ca-certificates \
#    && apt-key add /tmp/repo.corp.evenym.com-sig.key \
#    && echo "deb https://repo.corp.evernym.com/deb evernym-agency-dev-ubuntu main" >> /etc/apt/sources.list \
#    && apt-get update && apt-get install -y \
#        sovtoken \
#        sovtokenfees \
#    && rm -rf /var/lib/apt/lists/*

USER indy

RUN awk '{if (index($1, "NETWORK_NAME") != 0) {print("NETWORK_NAME = \"sandbox\"")} else print($0)}' /etc/indy/indy_config.py> /tmp/indy_config.py \
    && mv /tmp/indy_config.py /etc/indy/indy_config.py \
    && generate_indy_pool_transactions --nodes 4 --clients 5 --nodeNum 1 2 3 4 --ips="$pool_ip,$pool_ip,$pool_ip,$pool_ip" \
    && chmod -R a+rw /var/lib/indy /var/log/indy /etc/indy

USER root

COPY supervisord.conf /etc/supervisord.conf
COPY libsovtoken-ci-entrypoint.sh /usr/local/bin/

ENV LIBSOVTOKEN_CI_ENV_VERSION=0.12.0

ENTRYPOINT ["libsovtoken-ci-entrypoint.sh"]
