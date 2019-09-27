ARG ANDROID_NDK_DIR=/tmp/android/android_ndk
ARG ANDROID_NDK_VERSION=r20

FROM sovrin/dockerbase:rust-xenial-0.12.0
# TODO LABEL maintainer="Name <email-address>"

ARG ANDROID_NDK_VERSION
ARG ANDROID_NDK_DIR

COPY android-ndk-${ANDROID_NDK_VERSION}-linux-x86_64.zip ${ANDROID_NDK_DIR}/
RUN chmod -R 777 ${ANDROID_NDK_DIR}


############################################


FROM sovrin/dockerbase:rust-xenial-0.12.0
# TODO LABEL maintainer="Name <email-address>"

ARG PYTHON3_VERSION
ARG ANDROID_NDK_VERSION
ARG ANDROID_NDK_DIR

# python3
ENV PYTHON3_VERSION=${PYTHON3_VERSION:-3.5}
RUN apt-get update && apt-get install -y --no-install-recommends \
        python${PYTHON3_VERSION} \
        python3-pip \
    && rm -rf /var/lib/apt/lists/*

# android ndk
ENV ANDROID_NDK_VERSION=${ANDROID_NDK_VERSION}
ENV ANDROID_NDK_DIR=${ANDROID_NDK_DIR}
RUN mkdir -p ${ANDROID_NDK_DIR} && \
    chmod 777 ${ANDROID_NDK_DIR}
COPY --from=0 ${ANDROID_NDK_DIR} ${ANDROID_NDK_DIR}

COPY android-ndk-install /usr/local/bin/
RUN chmod +x /usr/local/bin/android-ndk-install

# TODO CMD ENTRYPOINT ...

ENV LIBSOVTOKEN_ANDROID_NDK_ENV_VERSION=0.7.0
