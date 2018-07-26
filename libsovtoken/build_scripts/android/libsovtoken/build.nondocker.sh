#!bin/bash

TARGET_API=$(grep api ../android_settings.txt | cut -d '=' -f 2)
TARGET_NDK=$(grep ndk ../android_settings.txt | cut -d '=' -f 2)
PREBUILT="${PWD}/android-dependencies"
FILEY_URL="https://repo.corp.evernym.com/filely/android/"

GREEN="[0;32m"
BLUE="[0;34m"
NC="[0m"
ESCAPE="\033"
UNAME=$(uname | tr '[:upper:]' '[:lower:]')
NDK=${TARGET_NDK}-${UNAME}-$(uname -m)

download_and_unzip_dependency() {
    _FILEY_NAME=$(grep "$1" $2)
    command pushd ${PREBUILT} > /dev/null
    echo -e "${ESCAPE}${GREEN}Downloading $1 prebuilt binaries"
    wget --no-check-certificate -q https://repo.corp.evernym.com/filely/android/${_FILEY_NAME}
    unzip -o -qq ${_FILEY_NAME}
    rm -f ${_FILEY_NAME}
    command popd > /dev/null
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

export ANDROID_NDK_ROOT="${PWD}/${UNAME}-${TARGET_NDK}"
export PKG_CONFIG_ALLOW_CROSS=1

if [ ! -d "${ANDROID_NDK_ROOT}" ] ; then
    if [ ! -f "${NDK}.zip" ] ; then
        echo -e "${ESCAPE}${GREEN}Downloading ${NDK}${ESCAPE}${NC}"
        wget -q https://dl.google.com/android/repository/${NDK}.zip
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
    export OPENSSL_DIR=${PREBUILT}/openssl_prebuilt/${target}
    export SODIUM_LIB_DIR=${PREBUILT}/sodium_prebuilt/${target}/lib
    export SODIUM_INCLUDE_DIR=${PREBUILT}/sodium_prebuilt/${target}/include
    export LIBZMQ_LIB_DIR=${PREBUILT}/zmq_prebuilt/${target}/lib
    export LIBZMQ_INCLUDE_DIR=${PREBUILT}/zmq_prebuilt/${target}/include
    export LIBINDY_DIR=${PREBUILT}/libindy/${CROSS_COMPILE}
    arch=${target}
    if [ ${target} = "armv7" ] ; then
        arch="arm"
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
        python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py --arch ${arch} --api ${TARGET_API} --install-dir ${TOOLCHAIN_DIR} || exit 1
    fi

    check=$(grep "\[target\.${CROSS_COMPILE}\]" ${HOME}/.cargo/config)
    if [ -z "${check}" ] ; then
        cat >> ${HOME}/.cargo/config <<EOF

[target.${CROSS_COMPILE}]
ar = "${AR}"
linker = "${CC}"
EOF
    fi
    check=$(rustup show | grep ${target})
    if [ -z "${check}" ] ; then
        rustup target add ${CROSS_COMPILE}
    fi
    if [ -d "${OPENSSL_DIR}" ] ; then
        echo "Found ${OPENSSL_DIR}"
    else
        download_and_unzip_dependency "openssl" "../libindy.dependencies.txt"
    fi

    if [ -d "${SODIUM_LIB_DIR}" ] ; then
        echo "Found ${SODIUM_LIB_DIR}"
    else
        download_and_unzip_dependency "sodium" "../libindy.dependencies.txt"
    fi
    if [ -d "${LIBZMQ_LIB_DIR}" ] ; then
        echo "Found ${LIBZMQ_LIB_DIR}"
    else
        download_and_unzip_dependency "zmq" "../libindy.dependencies.txt"
    fi
    if [ -d "${LIBINDY_DIR}" ] ; then
        echo "Found ${LIBINDY_DIR}"
    else
        download_and_unzip_dependency "libindy" "libsovtoken.dependencies.txt"
    fi

    command pushd ../../../.. > /dev/null
    cargo build --release --target=${CROSS_COMPILE}
    mv target/${CROSS_COMPILE}/release/libsovtoken.so target/${CROSS_COMPILE}/
    mv target/${CROSS_COMPILE}/release/libsovtoken.a target/${CROSS_COMPILE}/
    command popd > /dev/null
done

BUILD_TIME=$(date -u "+%Y%m%d%H%M")
GIT_REV=$(git --git-dir indy-sdk/.git rev-parse --short HEAD)
command pushd ../../../../target > /dev/null
zip "libsovtoken_${BUILD_TIME}-${GIT_REV}_android.zip" find . -type f \( -name "libsovtoken.so" -o -name "libsovtoken.a" \) -not -path "./*/deps/*"
command popd > /dev/null
