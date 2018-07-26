#!/bin/bash

TARGET_ARCH=$1
TARGET_API=21
CROSS_COMPILE=""
GIT_INSTALL=${2:-master}
INDY_PREBUILT="indy-android-dependencies/prebuilt"

download_and_unzip_dependency() {
    if [ ! -d "indy-android-dependencies" ] ; then
        git clone --depth 1 https://github.com/evernym/indy-android-dependencies.git
    fi
    command pushd ${INDY_PREBUILT}/$1 > /dev/null
    PREFIX=$(ls | grep "$1_$2.zip")
    PREFIX=${PREFIX/.zip/}
    if [ ! -d "${PREFIX}" ] ; then
        unzip -o -qq "${PREFIX}.zip"
    fi
    command popd > /dev/null
}

if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "Missing TARGET_ARCH argument"
    echo STDERR "e.g. x86 or arm"
    exit 1
fi

case ${TARGET_ARCH} in
    arm) CROSS_COMPILE="arm-linux-androideabi" ;;
    arm64) CROSS_COMPILE="aarch64-linux-android" ;;
    x86) CROSS_COMPILE="i686-linux-android" ;;
    x86_64) CROSS_COMPILE="x86_64-linux-android" ;;
    \?) echo STDERR "Unknown TARGET_ARCH"
        exit 1
        ;;
esac

if [ -z "${GIT_INSTALL}" ] ; then
    echo STDERR "Missing GIT_INSTALL argument"
    echo STDERR "e.g. master or rc or tags/v1.4.0"
    exit 1
fi

if [ -z "${ANDROID_OPENSSL_DIR}" ]; then
    ANDROID_OPENSSL_DIR="openssl_${TARGET_ARCH}"
    if [ -d "${ANDROID_OPENSSL_DIR}" ] ; then
        echo "Found ${ANDROID_OPENSSL_DIR}"
    elif [ -z "$3" ]; then
        if [ ! -d "${INDY_PREBUILT}/openssl/openssl_${TARGET_ARCH}" ] ; then
            download_and_unzip_dependency "openssl" "${TARGET_ARCH}"
        fi
        ANDROID_OPENSSL_DIR="${INDY_PREBUILT}/openssl/openssl_${TARGET_ARCH}"
    else
        ANDROID_OPENSSL_DIR=$3
    fi
fi

if [ -z "${ANDROID_SODIUM_DIR}" ]; then
    ANDROID_SODIUM_DIR="libsodium_${TARGET_ARCH}"
    if [ -d "${ANDROID_SODIUM_DIR}" ] ; then
        echo "Found ${ANDROID_SODIUM_DIR}"
    elif [ -z "$5" ]; then
        if [ ! -d "${INDY_PREBUILT}/sodium/libsodium_${TARGET_ARCH}" ] ; then
            download_and_unzip_dependency "sodium" "${TARGET_ARCH}"
        fi
        ANDROID_SODIUM_DIR="${INDY_PREBUILT}/sodium/libsodium_${TARGET_ARCH}"
    else
        ANDROID_SODIUM_DIR=$4
    fi
fi

if [ -z "${ANDROID_LIBZMQ_DIR}" ] ; then
    ANDROID_LIBZMQ_DIR="libzmq_${TARGET_ARCH}"
    if [ -d "${ANDROID_LIBZMQ_DIR}" ] ; then
        echo "Found ${ANDROID_LIBZMQ_DIR}"
    elif [ -z "$6" ] ; then
        if [ ! -d "${INDY_PREBUILT}/zmq/libzmq_${TARGET_ARCH}" ] ; then
            download_and_unzip_dependency "zmq" "${TARGET_ARCH}"
        fi
        ANDROID_LIBZMQ_DIR="${INDY_PREBUILT}/zmq/libzmq_${TARGET_ARCH}"
    else
        ANDROID_LIBZMQ_DIR=$5
    fi
fi

if [ ! -f "android-ndk-r16b-linux-x86_64.zip" ] ; then
    echo "Downloading android-ndk-r16b-linux-x86_64.zip"
    wget -q https://dl.google.com/android/repository/android-ndk-r16b-linux-x86_64.zip
fi

_INDY_SDK_REPO="https://github.com/hyperledger/indy-sdk.git"

if [ ! -d "indy-sdk" ] ; then
    echo "git cloning indy-sdk"

    git clone --branch ${GIT_INSTALL} ${_INDY_SDK_REPO}

    command pushd indy-sdk > /dev/null
    git checkout `cat ../../libindy.commit.sha1.hash.txt`
    command popd > /dev/null
else
    echo "Skipping git clone of indy-sdk"
    _GIT_COMMIT=$(cat ../../libindy.commit.sha1.hash.txt)
    _GIT_HEAD=$(git --git-dir indy-sdk/.git log -1 | head -n 1 | cut -d ' ' -f 2)
    echo "Current head set to ${_GIT_HEAD}"
    _MATCH=$(echo "${_GIT_HEAD}" | grep "${_GIT_COMMIT}")

    if [ -z "${_MATCH}" ] ; then
        echo STDERR "Branch is not set properly in indy-sdk/.git"
        exit 1
    fi
fi
rm -f "indy-sdk/libindy/Cargo.lock"
DOCKER_IMAGE_ID=$(docker image ls | grep libindy-android-${TARGET_ARCH})
if [ -z "${DOCKER_IMAGE_ID}" ] ; then
    docker build -t libindy-android-${TARGET_ARCH}:latest . --build-arg target_arch=${TARGET_ARCH} --build-arg target_api=${TARGET_API} --build-arg cross_compile=${CROSS_COMPILE} --build-arg openssl_dir=${ANDROID_OPENSSL_DIR} --build-arg sodium_dir=${ANDROID_SODIUM_DIR} --build-arg libzmq_dir=${ANDROID_LIBZMQ_DIR}
fi
docker run --rm --user indy_user -v ${PWD}/indy-sdk:/indy-sdk -w /indy-sdk/libindy -t libindy-android-${TARGET_ARCH}:latest cargo build --release --target=${CROSS_COMPILE}
