#!/usr/bin/env bash

abspath() {
    perl -e 'use Cwd "abs_path"; print abs_path(shift)' $1
}

TARGET_API=$(grep api ../android_settings.txt | cut -d '=' -f 2)
TARGET_NDK=$(grep ndk ../android_settings.txt | cut -d '=' -f 2)
PREBUILT="${PWD}/android-dependencies"
FILEY_URL="https://repo.corp.evernym.com/filely/android/"
LIBSOVTOKEN_DIR=$(abspath ${PWD}/../../..)

GREEN="[0;32m"
BLUE="[0;34m"
NC="[0m"
ESCAPE="\033"
UNAME=$(uname | tr '[:upper:]' '[:lower:]')
NDK=${TARGET_NDK}-${UNAME}-$(uname -m)

while getopts ":d" opt; do
    case ${opt} in
        d) export DOWNLOAD_PREBUILTS="1";;
        \?);;
    esac
done
shift $((OPTIND -1))


download_libindy(){
    #$1 Branch
    #$2 Version
    #$3 Arch
    command pushd ${PREBUILT} > /dev/null
        curl -sSLO https://repo.sovrin.org/android/libindy/$1/$2/libindy_android_$3_$2.zip
        unzip -o -qq "libindy_android_$3_$2.zip"
        rm "libindy_android_$3_$2.zip"
        export LIBINDY_DIR=${PREBUILT}/libindy_${target}/lib
    command popd > /dev/null
}

download_and_unzip_dependencies(){
    pushd ${PREBUILT}
        echo -e "${ESCAPE}${GREEN}Downloading openssl for $1 ${ESCAPE}${NC}"
        curl -sSLO https://repo.sovrin.org/android/libindy/deps/openssl/openssl_$1.zip
        unzip -o -qq openssl_$1.zip
        export OPENSSL_DIR=${PREBUILT}/openssl_${arch}

        echo -e "${ESCAPE}${GREEN}Downloading sodium for $1 ${ESCAPE}${NC}"
        curl -sSLO https://repo.sovrin.org/android/libindy/deps/sodium/libsodium_$1.zip
        unzip -o -qq libsodium_$1.zip
        export SODIUM_DIR=${PREBUILT}/libsodium_${arch}
        export SODIUM_LIB_DIR=${PREBUILT}/libsodium_${arch}/lib
        export SODIUM_INCLUDE_DIR=${PREBUILT}/libsodium_${arch}/include

        echo -e "${ESCAPE}${GREEN}Downloading zmq for $1 ${ESCAPE}${NC}"
        curl -sSLO https://repo.sovrin.org/android/libindy/deps/zmq/libzmq_$1.zip
        unzip -o -qq libzmq_$1.zip
        export LIBZMQ_DIR=${PREBUILT}/libzmq_${arch}

        rm openssl_$1.zip
        rm libsodium_$1.zip
        rm libzmq_$1.zip
    popd
}

get_cross_compile() {
    case $1 in
        arm)    echo "arm-linux-androideabi" ;;
        armv7)  echo "armv7-linux-androideabi" ;;
        arm64)  echo "aarch64-linux-android" ;;
        x86)    echo "i686-linux-android" ;;
        x86_64) echo "x86_64-linux-android" ;;
        \?) echo STDERR "Unknown TARGET_ARCH"
            exit 1
        ;;
    esac
}

if [ $# -gt 1 ] ; then
    archs=$@
else
    archs=(arm arm64 x86)
fi

mkdir -p ${PREBUILT}

export ANDROID_NDK_ROOT="${PWD}/${UNAME}-${TARGET_NDK}"
export PKG_CONFIG_ALLOW_CROSS=1

mkdir -p "${LIBSOVTOKEN_DIR}/.cargo/"

if [ ! -d "${ANDROID_NDK_ROOT}" ] ; then
    if [ ! -f "${NDK}.zip" ] ; then
        echo -e "${ESCAPE}${GREEN}Downloading ${NDK}${ESCAPE}${NC}"
        curl -sSLO https://dl.google.com/android/repository/${NDK}.zip || exit 1
    fi
    if [ ! -f "${NDK}.zip" ] ; then
        echo STDERR "Can't find ${NDK}"
        exit 1
    fi
    echo -e "${ESCAPE}${GREEN}Extracting ${NDK}${ESCAPE}${NC}"
    unzip -o -qq ${NDK}.zip
    mv ${TARGET_NDK} ${UNAME}-${TARGET_NDK}
fi

for target in ${archs[@]}; do
    export CROSS_COMPILE=$(get_cross_compile ${target})

    arch=${target}
    if [ ${target} = "armv7" ] ; then
        arch="arm"
    fi

    if [ ${arch} = "arm" ] || [ ${arch} = "x86" ]; then
	    TARGET_API=16
    else
	    TARGET_API=21
    fi
    export TOOLCHAIN_DIR=${PWD}/${UNAME}-${arch}

    ARCH_CROSS=$(get_cross_compile ${arch})

    export CC=${TOOLCHAIN_DIR}/bin/${ARCH_CROSS}-clang
    export AR=${TOOLCHAIN_DIR}/bin/${ARCH_CROSS}-ar
    export CXX=${TOOLCHAIN_DIR}/bin/${ARCH_CROSS}-clang++
    export CXXLD=${TOOLCHAIN_DIR}/bin/${ARCH_CROSS}-ld
    export RANLIB=${TOOLCHAIN_DIR}/bin/${ARCH_CROSS}-ranlib

    if [ ! -d "${TOOLCHAIN_DIR}" ] ; then
        echo -e "${ESCAPE}${BLUE}Making standalone toolchain for ${target}${ESCAPE}${NC}"
        python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py --arch ${arch} --stl gnustl --api ${TARGET_API} --install-dir ${TOOLCHAIN_DIR} || exit 1
    fi

    cat > ${LIBSOVTOKEN_DIR}/.cargo/config <<EOF
[target.${CROSS_COMPILE}]
ar = "${AR}"
linker = "${CC}"
EOF
    check=$(rustup show | grep ${CROSS_COMPILE})
    if [ -z "${check}" ] ; then
        rustup target add ${CROSS_COMPILE}
    fi

    if [ "${DOWNLOAD_PREBUILTS}" == "1" ]; then
        download_and_unzip_dependencies ${target}
    fi

    if [ -d "${OPENSSL_DIR}" ] || [ -z "${OPENSSL_DIR}" ] ; then
        echo -e "${ESCAPE}${BLUE}Found ${OPENSSL_DIR}${ESCAPE}${NC}"
    else
        echo "${ESCAPE}${RED}OPENSSL_DIR not found${ESCAPE}${NC}"
        exit 1
    fi

    if [ -d "${SODIUM_LIB_DIR}" ] || [ -z "${SODIUM_LIB_DIR}" ] ; then
        echo -e "${ESCAPE}${BLUE}Found ${SODIUM_LIB_DIR}${ESCAPE}${NC}"
    else
        echo "${ESCAPE}${RED}SODIUM_LIB_DIR not found${ESCAPE}${NC}"
        exit 1
    fi
    if [ -d "${LIBINDY_DIR}" ] ; then
        echo -e "${ESCAPE}${BLUE}Found ${LIBINDY_DIR}${ESCAPE}${NC}"
    else
        libindy_version=$(grep libindy libsovtoken.dependencies.txt | cut -d '=' -f 2)
        libindy_branch=$(grep libindy libsovtoken.dependencies.txt | cut -d '=' -f 3)
        download_libindy ${libindy_branch} ${libindy_version} ${target}
    fi

    command pushd ${LIBSOVTOKEN_DIR} > /dev/null
    cargo build --release --target=${CROSS_COMPILE}
    unset LIBINDY_DIR
    for filename in libsovtoken.so libsovtoken.a; do
        if [ -f "target/${CROSS_COMPILE}/release/${filename}" ] ; then
            mv target/${CROSS_COMPILE}/release/${filename} target/${CROSS_COMPILE}/
        else
            echo STDERR "Build didn't complete for ${arch}"
            exit 1
        fi
    done
    command popd > /dev/null
done

BUILD_TIME=$(date -u "+%Y%m%d%H%M")
GIT_REV=$(git rev-parse --short HEAD)
command pushd ${LIBSOVTOKEN_DIR} > /dev/null
LIBSOVTOKEN_VER=$(grep ^version Cargo.toml | head -n 1 | cut -d '"' -f 2)
mv target libsovtoken
zip -qq "libsovtoken_${LIBSOVTOKEN_VER}-${BUILD_TIME}-${GIT_REV}_all.zip" `find libsovtoken -type f \( -name "libsovtoken.so" -o -name "libsovtoken.a" \) | grep android | egrep -v 'deps|debug|release'`
mv libsovtoken target
command popd > /dev/null
mv ${LIBSOVTOKEN_DIR}/"libsovtoken_${LIBSOVTOKEN_VER}-${BUILD_TIME}-${GIT_REV}_all.zip" .
