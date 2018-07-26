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
    arm)    CROSS_COMPILE="armv7-linux-androideabi" ;;
    arm64)  CROSS_COMPILE="aarch64-linux-android" ;;
    x86)    CROSS_COMPILE="i686-linux-android" ;;
    x86_64) CROSS_COMPILE="x86_64-linux-android" ;;
    \?) echo STDERR "Unknown TARGET_ARCH"
        exit 1
        ;;
esac

if [ -z "${DOCKER_IMAGE_ID}" ] ; then
    if [ -z "${ANDROID_LIBINDY_DIR}" ] ; then
        ANDROID_LIBINDY_DIR="libindy_${TARGET_ARCH}"
        if [ -d "${ANDROID_LIBINDY_DIR}" ] ; then
            echo "Found ${ANDROID_LIBINDY_DIR}"
        elif [ -z "$2" ] ; then
            mkdir -p ${ANDROID_LIBINDY_DIR}
            command pushd ${ANDROID_LIBINDY_DIR} > /dev/null

            LIBINDY_FILE="$3"
            if [ -z "${LIBINDY_FILE}" ] ; then
                LIBINDY_FILE=$(curl -s https://repo.corp.evernym.com/filely/android/ | grep libindy | egrep "${TARGET_ARCH}\>" | cut -d '"' -f 2 | sort -r | head -n 1)
            fi

            if [ -z "${LIBINDY_FILE}" ] ; then
                echo "Unable to download prebuilt libindy"
                exit 1
            fi
            wget -q --no-check-certificate "https://repo.corp.evernym.com/filely/android/${LIBINDY_FILE}"
            if [ ! -f ${LIBINDY_FILE} ] ; then
                echo "Unable to download file from https://repo.corp.evernym.com/filely/android/${LIBINDY_FILE}"
                exit 1
            fi
            unzip -qq ${LIBINDY_FILE}
            command popd > /dev/null
            #echo STDERR "Missing ANDROID_LIBINDY_DIR argument and environment variable"
            #echo STDERR "e.g. set ANDROID_LIBINDY_DIR=<path> for environment or libindy_${TARGET_ARCH}"
            #exit 1
        else
            ANDROID_LIBINDY_DIR=$2
        fi
    fi

    if [ ! -f "${ANDROID_LIBINDY_DIR}/libindy.a" ] ; then
        echo "${ANDROID_LIBINDY_DIR}/libindy.a does not exist"
        exit 1
    fi
    if [ ! -f "${ANDROID_LIBINDY_DIR}/libindy.so" ] ; then
        echo "${ANDROID_LIBINDY_DIR}/libindy.so does not exist"
        exit 1
    fi
    
    if [ -z "${ANDROID_OPENSSL_DIR}" ]; then
        ANDROID_OPENSSL_DIR="openssl_${TARGET_ARCH}"
        if [ -d "${ANDROID_OPENSSL_DIR}" ] ; then
            echo "Found ${ANDROID_OPENSSL_DIR}"
        elif [ -z "$4" ]; then
            if [ ! -d "${INDY_PREBUILT}/openssl/openssl_${TARGET_ARCH}" ] ; then
                download_and_unzip_dependency "openssl" "${TARGET_ARCH}"
            fi
            ANDROID_OPENSSL_DIR="${INDY_PREBUILT}/openssl/openssl_${TARGET_ARCH}"
        else
            ANDROID_OPENSSL_DIR=$4
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
            ANDROID_SODIUM_DIR=$5
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
            ANDROID_LIBZMQ_DIR=$6
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
    
    docker build -t libsovtoken-android-${TARGET_ARCH}:latest . --build-arg target_arch=${TARGET_ARCH} --build-arg target_api=${TARGET_API} --build-arg cross_compile=${CROSS_COMPILE} --build-arg openssl_dir=${ANDROID_OPENSSL_DIR} --build-arg libsodium_dir=${ANDROID_SODIUM_DIR} --build-arg libzmq_dir=${ANDROID_LIBZMQ_DIR} --build-arg libindy_dir=${ANDROID_LIBINDY_DIR}
    rm -rf playground
fi

docker run --user sovtoken_user:sovtoken_user --rm -v ${LIBSOVTOKEN_DIR}:/data -w /data -t libsovtoken-android-${TARGET_ARCH}:latest cargo build --release --target ${CROSS_COMPILE}
