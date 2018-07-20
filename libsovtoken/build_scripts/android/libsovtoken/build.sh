#!/bin/bash

TARGET_ARCH=$1
TARGET_API=21
CROSS_COMPILE=""
INDY_PREBUILT="indy-android-dependencies/prebuilt"
LIBSOVTOKEN_DIR=$(perl -e 'use Cwd "abs_path"; print abs_path(shift)' "${PWD}/../../../")
DOCKER_IMAGE_ID=$(docker image ls | grep libsovtoken-android-${TARGET_ARCH})

download_and_unzip_dependency() {
    if [ ! -d "indy-android-dependencies" ] ; then
        git clone --depth 1 https://github.com/evernym/indy-android-dependencies.git
    fi
    pushd ${INDY_PREBUILT}/$1
    PREFIX=$(ls | grep "$1_$2.zip")
    PREFIX=${PREFIX/.zip/}
    if [ ! -d "${PREFIX}" ] ; then
        unzip -o -qq "${PREFIX}.zip"
    fi
    popd
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
    \?) echo STDERR "Unknown TARGET_ARCH"
        exit 1
        ;;
esac

if [ -z "${DOCKER_IMAGE_ID}" ] ; then
    if [ -z "${LIBINDY_DIR}" ] ; then
        LIBINDY_DIR="libindy_${TARGET_ARCH}"
        if [ -d "${LIBINDY_DIR}" ] ; then
            echo "Found ${LIBINDY_DIR}"
        elif [ -z "$2" ] ; then
            echo STDERR "Missing LIBINDY_DIR argument and environment variable"
            echo STDERR "e.g. set LIBINDY_DIR=<path> for environment or libindy_${TARGET_ARCH}"
            exit 1
        else
            LIBINDY_DIR=$2
        fi
    fi
    
    if [ -z "${OPENSSL_DIR}" ]; then
        OPENSSL_DIR="openssl_${TARGET_ARCH}"
        if [ -d "${OPENSSL_DIR}" ] ; then
            echo "Found ${OPENSSL_DIR}"
        elif [ -z "$3" ]; then
            if [ ! -d "${INDY_PREBUILT}/openssl/openssl_${TARGET_ARCH}" ] ; then
                download_and_unzip_dependency "openssl" "${TARGET_ARCH}"
            fi
            OPENSSL_DIR="${INDY_PREBUILT}/openssl/openssl_${TARGET_ARCH}"
        else
            OPENSSL_DIR=$3
        fi
    fi
    
    if [ -z "${SODIUM_DIR}" ]; then
        SODIUM_DIR="libsodium_${TARGET_ARCH}"
        if [ -d "${SODIUM_DIR}" ] ; then
            echo "Found ${SODIUM_DIR}"
        elif [ -z "$4" ]; then
            if [ ! -d "${INDY_PREBUILT}/sodium/libsodium_${TARGET_ARCH}" ] ; then
                download_and_unzip_dependency "sodium" "${TARGET_ARCH}" 
            fi
            SODIUM_DIR="${INDY_PREBUILT}/sodium/libsodium_${TARGET_ARCH}"
        else
            SODIUM_DIR=$4
        fi    
    fi
    
    if [ -z "${LIBZMQ_DIR}" ] ; then
        LIBZMQ_DIR="libzmq_${TARGET_ARCH}" 
        if [ -d "${LIBZMQ_DIR}" ] ; then
            echo "Found ${LIBZMQ_DIR}"
        elif [ -z "$5" ] ; then
            if [ ! -d "${INDY_PREBUILT}/zmq/libzmq_${TARGET_ARCH}" ] ; then
                download_and_unzip_dependency "zmq" "${TARGET_ARCH}" 
            fi
            LIBZMQ_DIR="${INDY_PREBUILT}/zmq/libzmq_${TARGET_ARCH}"
        else
            LIBZMQ_DIR=$5
        fi
    fi
    
    if [ ! -f "android-ndk-r16b-linux-x86_64.zip" ] ; then
        echo "Downloading android-ndk-r16b-linux-x86_64.zip"
        wget -q https://dl.google.com/android/repository/android-ndk-r16b-linux-x86_64.zip 
    fi

    if [ -d "playground" ]  ; then
        rm -rf playground
    fi
    cargo new --lib playground
    cp ${LIBSOVTOKEN_DIR}/Cargo.toml playground/
    cp ${LIBSOVTOKEN_DIR}/build.rs playground/
    
    docker build -t libsovtoken-android-${TARGET_ARCH}:latest . --build-arg target_arch=${TARGET_ARCH} --build-arg target_api=${TARGET_API} --build-arg cross_compile=${CROSS_COMPILE} --build-arg openssl_dir=${OPENSSL_DIR} --build-arg libsodium_dir=${SODIUM_DIR} --build-arg libzmq_dir=${LIBZMQ_DIR} --build-arg libindy_dir=${LIBINDY_DIR}
    rm -rf playground
fi

docker run --user sovtoken_user:sovtoken_user --rm -v ${LIBSOVTOKEN_DIR}:/data -w /data -t libsovtoken-android-${TARGET_ARCH}:latest cargo build --release --target ${CROSS_COMPILE}
