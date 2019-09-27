ARG ANDROID_PREBUILT_DIR=/tmp/android/libsovtoken_prebuilt

FROM sovrin/libsovtoken:android_ndk-xenial-0.7.0
# TODO LABEL maintainer="Name <email-address>"

ARG ANDROID_PREBUILT_DIR

COPY *.zip ${ANDROID_PREBUILT_DIR}/
RUN chmod -R 777 ${ANDROID_PREBUILT_DIR}


############################################


FROM sovrin/libsovtoken:android_ndk-xenial-0.7.0
# TODO LABEL maintainer="Name <email-address>"

ARG u_id=1000
ARG u_name=user

ARG ANDROID_ARCHS
ARG ANDROID_PREBUILT_DIR
ARG RUST_TARGETS

RUN if [ "$u_id" != "0" ]; then \
        useradd -ms /bin/bash -u $u_id $u_name; \
    fi

ENV TEST_USER_UID=$u_id

ENV ANDROID_ARCHS=${ANDROID_ARCHS:-"arm armv7 arm64 x86 x86_64"}
ENV ANDROID_PREBUILT_DIR=${ANDROID_PREBUILT_DIR}

RUN mkdir -p ${ANDROID_PREBUILT_DIR} && \
    chmod 777 ${ANDROID_PREBUILT_DIR}
COPY --from=0 ${ANDROID_PREBUILT_DIR} ${ANDROID_PREBUILT_DIR}

# TODO fill cargo cache to speed up docker containers
COPY Cargo.toml /tmp/_libsovtoken/
RUN chown -R $u_id:$u_id /tmp/_libsovtoken/

USER $u_id
RUN cd /tmp/_libsovtoken \
    && cargo update \
    && rm -rf /tmp/_libsovtoken/


ENV RUST_TARGETS=${RUST_TARGETS:-"arm-linux-androideabi armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android"}
#RUN rustup target add ${RUST_TARGETS} # This will increase size of the image for about 350Mb

# TODO some entrypoint

ENV LIBSOVTOKEN_ANDROID_BUILD_ENV_VERSION=0.7.0
