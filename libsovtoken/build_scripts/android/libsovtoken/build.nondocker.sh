#!/usr/bin/env bash

abspath() {
    perl -e 'use Cwd "abs_path"; print abs_path(shift)' $1
}

TARGET_NDK=$(grep ndk ../android_settings.txt | cut -d '=' -f 2)

BUILD_DIR=${BUILD_DIR:-${PWD}/_build}
PREBUILT=${PREBUILT:-"${BUILD_DIR}/android-dependencies"}
TARGET_DIR=${TARGET_DIR:-${BUILD_DIR}/libsovtoken}

LIBSOVTOKEN_DIR=$(abspath ${PWD}/../../..)

if [ -z "${CARGO_TARGET_DIR+x}" ]; then
    _CARGO_TARGET_DIR=target
else
    _CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-.}"
fi


GREEN="[0;32m"
BLUE="[0;34m"
NC="[0m"
ESCAPE="\033"
UNAME=$(uname | tr '[:upper:]' '[:lower:]')

while getopts ":ds" opt; do
    case ${opt} in
        d) DOWNLOAD_PREBUILTS="1";;
        s) SKIP_PACKAGING="1";;
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
    command popd > /dev/null
}

download_and_unzip_dependencies(){
    pushd ${PREBUILT}
        echo -e "${ESCAPE}${GREEN}Downloading openssl for $1 ${ESCAPE}${NC}"
        curl -sSLO https://repo.sovrin.org/android/libindy/deps/openssl/openssl_$1.zip
        unzip -o -qq openssl_$1.zip

        echo -e "${ESCAPE}${GREEN}Downloading sodium for $1 ${ESCAPE}${NC}"
        curl -sSLO https://repo.sovrin.org/android/libindy/deps/sodium/libsodium_$1.zip
        unzip -o -qq libsodium_$1.zip

        rm openssl_$1.zip
        rm libsodium_$1.zip
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

if [ $# -gt 0 ] ; then
    archs=$@
else
    archs=(arm armv7 arm64 x86 x86_64)
fi

mkdir -p ${PREBUILT}
mkdir -p "$TARGET_DIR"

if [ -z "$ANDROID_NDK_ROOT" ]; then
    export ANDROID_NDK_ROOT="${BUILD_DIR}/${UNAME}-${TARGET_NDK}"
fi

export PKG_CONFIG_ALLOW_CROSS=1

if [ ! -d "${ANDROID_NDK_ROOT}" ] ; then
    NDK=${TARGET_NDK}-${UNAME}-$(uname -m)
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
    mv ${TARGET_NDK} ${ANDROID_NDK_ROOT}
fi

for target in ${archs[@]}; do
    export CROSS_COMPILE=$(get_cross_compile ${target})

    _TARGET_DIR=${TARGET_DIR}/${CROSS_COMPILE}
    rm -rf ${_TARGET_DIR}
    mkdir -p ${_TARGET_DIR}

    arch=${target}
    if [ ${target} = "armv7" ] ; then
        arch="arm"
    fi

    if [ ${arch} = "arm" ] || [ ${arch} = "x86" ]; then
	    TARGET_API=16
    else
	    TARGET_API=21
    fi
    export TOOLCHAIN_DIR=${BUILD_DIR}/${UNAME}-${arch}

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

    mkdir -p "${LIBSOVTOKEN_DIR}/.cargo/"
    cat > ${LIBSOVTOKEN_DIR}/.cargo/config <<EOF
[target.${CROSS_COMPILE}]
ar = "${AR}"
linker = "${CC}"
EOF
    check=$(rustup show | grep ${CROSS_COMPILE} || true)
    if [ -z "${check}" ] ; then
        rustup target add ${CROSS_COMPILE}
    fi

    if [ -z "${LIBINDY_DIR}" ]; then
        export LIBINDY_DIR=${PREBUILT}/libindy_${target}/lib
    fi
    if [ -z "${OPENSSL_DIR}" ]; then
        export OPENSSL_DIR=${PREBUILT}/openssl_${target}
    fi
    if [ -z "${SODIUM_DIR}" ]; then
        export SODIUM_DIR=${PREBUILT}/libsodium_${target}
    fi
    if [ -z "${SODIUM_LIB_DIR}" ]; then
        export SODIUM_LIB_DIR=${PREBUILT}/libsodium_${target}/lib
    fi
    if [ -z "${SODIUM_INCLUDE_DIR}" ]; then
        export SODIUM_INCLUDE_DIR=${PREBUILT}/libsodium_${target}/include
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
    cargo update
    cargo build -vv --release --target=${CROSS_COMPILE}
    rm -rf "${LIBSOVTOKEN_DIR}/.cargo"

    unset LIBINDY_DIR
    for filename in libsovtoken.so libsovtoken.a; do
        if [ -f "${_CARGO_TARGET_DIR}/${CROSS_COMPILE}/release/${filename}" ] ; then
            cp ${_CARGO_TARGET_DIR}/${CROSS_COMPILE}/release/${filename} ${_TARGET_DIR}
        else
            echo STDERR "Build didn't complete for ${target}"
            exit 1
        fi
    done
    command popd > /dev/null
done

if [ ! "${SKIP_PACKAGING}" == "1" ]; then
   . ./pack.sh "$TARGET_DIR"
else
    echo "Skipping packaging"
fi
