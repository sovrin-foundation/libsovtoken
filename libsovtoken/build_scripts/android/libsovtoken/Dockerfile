FROM ubuntu:16.04
LABEL maintainer="Michael Lodder <redmike7@gmail.com>"

ARG target_arch
ARG target_api
ARG cross_compile
ARG openssl_dir
ARG libsodium_dir
ARG libzmq_dir
ARG libindy_dir

ENV TARGET_ARCH ${target_arch}
ENV TARGET_API ${target_api}
ENV CROSS_COMPILE ${cross_compile}
ENV OPENSSL_DIR /home/sovtoken_user/openssl_${target_arch}
ENV SODIUM_LIB_DIR /home/sovtoken_user/libsodium_${target_arch}/lib
ENV SODIUM_INCLUDE_DIR /home/sovtoken_user/libsodium_${target_arch}/include
ENV LIBZMQ_LIB_DIR /home/sovtoken_user/libzmq_${target_arch}/lib
ENV LIBZMQ_INCLUDE_DIR /home/sovtoken_user/libzmq_${target_arch}/include
ENV LIBINDY_DIR /home/sovtoken_user/libindy_${target_arch}
ENV ANDROID_NDK_ROOT /home/sovtoken_user/android-ndk-r16b
ENV TOOLCHAIN_DIR /home/sovtoken_user/${target_arch}
ENV PATH ${TOOLCHAIN_DIR}/bin:${PATH}
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV CC=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang
ENV AR=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ar
ENV CXX=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang++
ENV CXXLD=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ld
ENV RANLIB=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ranlib
ENV TARGET=android
ENV PATH=/home/sovtoken_user/.cargo/bin:${PATH}

RUN apt-get -qq update -y \
    && apt-get -qq install -y sudo zip unzip libtool curl wget python3 2>&1 > /dev/null \
    && useradd -m -d /home/sovtoken_user -s /bin/bash -p $(openssl passwd -1 "token") sovtoken_user \
    && usermod -aG sudo sovtoken_user

USER sovtoken_user
WORKDIR /home/sovtoken_user
COPY android-ndk-r16b-linux-x86_64.zip /home/sovtoken_user/
COPY --chown=sovtoken_user:sovtoken_user ${openssl_dir}/ /home/sovtoken_user/openssl_${target_arch}/
COPY --chown=sovtoken_user:sovtoken_user ${libsodium_dir}/ /home/sovtoken_user/libsodium_${target_arch}/
COPY --chown=sovtoken_user:sovtoken_user ${libzmq_dir}/ /home/sovtoken_user/libzqm_${target_arch}/
COPY --chown=sovtoken_user:sovtoken_user ${libindy_dir}/ /home/sovtoken_user/libindy_${target_arch}/
COPY --chown=sovtoken_user:sovtoken_user playground/ /home/sovtoken_user/playground/
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && unzip -qq /home/sovtoken_user/android-ndk-r16b-linux-x86_64.zip -d /home/sovtoken_user/ \
    && python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py --arch ${TARGET_ARCH} --api ${TARGET_API} --install-dir ${TOOLCHAIN_DIR} \
    && rm -rf ${ANDROID_NDK_ROOT} \
    && echo "[target.${CROSS_COMPILE}]" > .cargo/config \
    && echo "ar = \"${AR}\"" >> .cargo/config \
    && echo "linker = \"${CC}\"" >> .cargo/config \
    && rustup target add ${CROSS_COMPILE} \
    && cd playground \
    && cargo build \
    ; cd .. \
    && rm -rf playground \
    && echo "libsovtoken android image successful"
