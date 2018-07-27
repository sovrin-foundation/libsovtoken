#!bin/bash

abspath() {
    perl -e 'use Cwd "abs_path"; print abs_path(shift)' $1
}

TARGET_API=$(grep api ../android_settings.txt | cut -d '=' -f 2)
TARGET_NDK=$(grep ndk ../android_settings.txt | cut -d '=' -f 2)
INDY_PREBUILT="${PWD}/indy-android-dependencies"
INDY_SDK_REPO="https://github.com/hyperledger/indy-sdk.git"
FILEY_URL="https://repo.corp.evernym.com/filely/android/"
LIBSOVTOKEN_DIR=$(abspath ${PWD}/../../..)

GREEN="[0;32m"
BLUE="[0;34m"
NC="[0m"
ESCAPE="\033"
UNAME=$(uname | tr '[:upper:]' '[:lower:]')
NDK=${TARGET_NDK}-${UNAME}-$(uname -m)

_GIT_COMMIT=$(cat ../../libindy.commit.sha1.hash.txt)
if [ ! -d "indy-sdk" ] ; then
    echo -e "${ESCAPE}${GREEN}Using commit ${_GIT_COMMIT}${ESCAPE}${NC}"
    if [ "${_GIT_COMMIT}" = "HEAD" ] ; then
        echo -e "${ESCAPE}${GREEN}Shallow clone${ESCAPE}${NC}"
        git clone --depth 1 --branch master ${INDY_SDK_REPO}
        rm -f "indy-sdk/libindy/Cargo.lock"
    else
        git clone --branch master ${INDY_SDK_REPO}
        rm -f "indy-sdk/libindy/Cargo.lock"
        command pushd indy-sdk > /dev/null
        git checkout ${_GIT_COMMIT}
        command popd > /dev/null
    fi
else
    if [ "${_GIT_COMMIT}" = "HEAD" ] ; then
        _GIT_HEAD=$(git --git-dir indy-sdk/.git branch | grep '\*')
        echo "Current head set to ${_GIT_HEAD}"
        _MATCH=$(echo "$_GIT_HEAD" | grep -v "detached")
    else
        _GIT_HEAD=$(git --git-dir indy-sdk/.git rev-parse HEAD)
        echo "Current head set to ${_GIT_HEAD}"
        _MATCH=$(echo "${_GIT_HEAD}" | grep "${_GIT_COMMIT}")
    fi
    if [ -z "${_MATCH}" ] ; then
        echo STDERR "Branch is not set properly in indy-sdk/.git"
        exit 1
    fi
fi

download_and_unzip_dependency() {
    _FILEY_NAME=$(grep "$1" ../libindy.dependencies.txt)
    command pushd ${INDY_PREBUILT} > /dev/null
    echo -e "${ESCAPE}${GREEN}Downloading $1 prebuilt binaries"
    wget --no-check-certificate -q https://repo.corp.evernym.com/filely/android/${_FILEY_NAME} || exit 1
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

mkdir -p ${INDY_PREBUILT}

export ANDROID_NDK_ROOT="${PWD}/${UNAME}-${TARGET_NDK}"
export PKG_CONFIG_ALLOW_CROSS=1

mkdir -p "${LIBSOVTOKEN_DIR}/.cargo/"

if [ ! -d "${ANDROID_NDK_ROOT}" ] ; then
    if [ ! -f "${NDK}.zip" ] ; then
        echo -e "${ESCAPE}${GREEN}Downloading ${NDK}${ESCAPE}${NC}"
        wget -q https://dl.google.com/android/repository/${NDK}.zip || exit 1
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
    export OPENSSL_DIR=${INDY_PREBUILT}/openssl_prebuilt/${target}
    export SODIUM_LIB_DIR=${INDY_PREBUILT}/sodium_prebuilt/${target}/lib
    export SODIUM_INCLUDE_DIR=${INDY_PREBUILT}/sodium_prebuilt/${target}/include
    export LIBZMQ_LIB_DIR=${INDY_PREBUILT}/zmq_prebuilt/${target}/lib
    export LIBZMQ_INCLUDE_DIR=${INDY_PREBUILT}/zmq_prebuilt/${target}/include
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

    cat > ${LIBSOVTOKEN_DIR}/.cargo/config <<EOF
[target.${CROSS_COMPILE}]
ar = "${AR}"
linker = "${CC}"
EOF
    check=$(rustup show | grep ${CROSS_COMPILE})
    if [ -z "${check}" ] ; then
        rustup target add ${CROSS_COMPILE}
    fi
    if [ -d "${OPENSSL_DIR}" ] ; then
        echo "${ESCAPE}${BLUE}Found ${OPENSSL_DIR}${ESCAPE}${NC}"
    else
        download_and_unzip_dependency "openssl"
    fi

    if [ -d "${SODIUM_LIB_DIR}" ] ; then
        echo "${ESCAPE}${BLUE}Found ${SODIUM_LIB_DIR}${ESCAPE}${NC}"
    else
        download_and_unzip_dependency "sodium"
    fi
    if [ -d "${LIBZMQ_LIB_DIR}" ] ; then
        echo "${ESCAPE}${BLUE}Found ${LIBZMQ_LIB_DIR}${ESCAPE}${NC}"
    else
        download_and_unzip_dependency "zmq"
    fi

    command pushd indy-sdk/libindy > /dev/null
    cargo build --release --target=${CROSS_COMPILE}

    for filename in libindy.so libindy.a; do
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
GIT_REV=$(git --git-dir indy-sdk/.git rev-parse --short HEAD)
LIBINDY_VER=$(grep ^version indy-sdk/libindy/Cargo.toml | head -n 1 | cut -d '"' -f 2)
command pushd indy-sdk/libindy > /dev/null
mv target libindy
zip -qq "libindy_${LIBINDY_VER}-${BUILD_TIME}-${GIT_REV}_all.zip" `find libindy -type f \( -name "libindy.so" -o -name "libindy.a" \) | grep android | egrep -v 'deps|debug|release'`
mv libindy target
command popd > /dev/null
mv indy-sdk/libindy/"libindy_${LIBINDY_VER}-${BUILD_TIME}-${GIT_REV}_all.zip" .
