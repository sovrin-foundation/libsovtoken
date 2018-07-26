FROM evernym/libsovtoken:android_ndk-xenial-0.1.0
# TODO LABEL maintainer="Name <email-address>"

ARG u_id=1000
ARG u_name=user

RUN if [ "$u_id" != "0" ]; then \
        useradd -ms /bin/bash -u $u_id $u_name; \
    fi

ENV TEST_USER_UID=$u_id

RUN apt-get update \
    && apt-get install -y \
        libtool \
        libzmq3-dev \
    && rm -rf /var/lib/apt/lists/*

ENV BUILD_DIR=/tmp/build
ENV PREBUILT_DIR=$BUILD_DIR/prebuilt/
ENV INDY_SDK_DIR=$BUILD_DIR/indy-sdk

# fill cargo cache to speed up docker containers
COPY indy-sdk $INDY_SDK_DIR
RUN chown -R ${u_id}:${u_id} $BUILD_DIR

USER $u_id:$u_id
RUN cd $INDY_SDK_DIR/libindy \
    && cargo update

USER root
COPY openssl*.zip $PREBUILT_DIR
COPY libsodium*.zip $PREBUILT_DIR
COPY libzmq*.zip $PREBUILT_DIR
RUN chown -R ${u_id}:${u_id} $PREBUILT_DIR

COPY libindy-android-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/libindy-android-entrypoint.sh
ENTRYPOINT ["libindy-android-entrypoint.sh"]

ENV LIBSOVTOKEN_LIBINDY_ANDROID_ENV_VERSION=0.1.0
