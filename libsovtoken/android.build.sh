#!/usr/bin/env bash

WORKDIR=${PWD}
CI_DIR="${WORKDIR}/../devops"
BUILD_FOLDER="${WORKDIR}/../android_build"
DOWNLOAD_PREBUILTS=0
#export INDY_PREBUILT_DEPS_DIR=${HOME}/Work/repos/faisal00813/indy-sdk/android_build/libindy_x86/lib

while getopts ":d" opt; do
    case ${opt} in
        d) DOWNLOAD_PREBUILTS=1;;
        \?);;
    esac
done
shift $((OPTIND -1))

echo "DOWNLOAD_PREBUILTS=${DOWNLOAD_PREBUILTS}"
TARGET_ARCH=$1

if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "Missing TARGET_ARCH argument"
    echo STDERR "e.g. x86 or arm"
    exit 1
fi

source ${CI_DIR}/setup.android.env.sh

BUILD_LIBINDY=0

setup_dependencies(){
    if [ ${DOWNLOAD_PREBUILTS} -eq 1 ]; then
        download_and_unzip_dependencies_for_all_architectures
        else
            echo "not downloading prebuilt dependencies. Dependencies locations have to be passed"
            if [ -z "${OPENSSL_DIR}" ]; then
                OPENSSL_DIR="openssl_${TARGET_ARCH}"
                if [ -d "${OPENSSL_DIR}" ] ; then
                    echo "Found ${OPENSSL_DIR}"
                elif [ -z "$4" ]; then
                    echo STDERR "Missing OPENSSL_DIR argument and environment variable"
                    echo STDERR "e.g. set OPENSSL_DIR=<path> for environment or openssl_${TARGET_ARCH}"
                    exit 1
                else
                    OPENSSL_DIR=$4
                fi
            fi

            if [ -z "${SODIUM_DIR}" ]; then
                SODIUM_DIR="libsodium_${TARGET_ARCH}"
                if [ -d "${SODIUM_DIR}" ] ; then
                    echo "Found ${SODIUM_DIR}"
                elif [ -z "$5" ]; then
                    echo STDERR "Missing SODIUM_DIR argument and environment variable"
                    echo STDERR "e.g. set SODIUM_DIR=<path> for environment or libsodium_${TARGET_ARCH}"
                    exit 1
                else
                    SODIUM_DIR=$5
                fi
            fi
            if [ -z "${ZMQ_DIR}" ]; then
                ZMQ_DIR="ZMQ_${TARGET_ARCH}"
                if [ -d "${ZMQ_DIR}" ] ; then
                    echo "Found ${ZMQ_DIR}"
                elif [ -z "$5" ]; then
                    echo STDERR "Missing ZMQ_DIR argument and environment variable"
                    echo STDERR "e.g. set ZMQ_DIR=<path> for environment or zmq_${TARGET_ARCH}"
                    exit 1
                else
                    ZMQ_DIR=$5
                fi
            fi
    fi

    if [ -z "${INDY_DIR}" ]; then
        BUILD_LIBINDY=1
        INDY_DIR="${BUILD_FOLDER}/indy-sdk"
    fi
}

statically_link_dependencies_with_libindy(){
    $CC -v -shared -o${BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib/libindy.so -Wl,--whole-archive \
        ${WORKDIR}/target/${TRIPLET}/release/libindy.a \
        ${TOOLCHAIN_DIR}/sysroot/usr/lib/libz.so \
        ${TOOLCHAIN_DIR}/sysroot/usr/lib/libm.a \
        ${TOOLCHAIN_DIR}/sysroot/usr/lib/liblog.so \
        ${OPENSSL_DIR}/lib/libssl.a \
        ${OPENSSL_DIR}/lib/libcrypto.a \
        ${SODIUM_LIB_DIR}/libsodium.a \
        ${LIBZMQ_LIB_DIR}/libzmq.a \
        ${TOOLCHAIN_DIR}/${TOOLCHAIN_TRIPLET}/lib/libgnustl_shared.so \
        -Wl,--no-whole-archive -z muldefs
}

package_library(){

    mkdir -p ${BUILD_FOLDER}/libsovtoken_${TARGET_ARCH}/lib

    cp -rf "${WORKDIR}/include" ${BUILD_FOLDER}/libsovtoken_${TARGET_ARCH}
    cp "${WORKDIR}/target/${TRIPLET}/release/libsovtoken.a" ${BUILD_FOLDER}/libsovtoken_${TARGET_ARCH}/lib
    cp "${WORKDIR}/target/${TRIPLET}/release/libsovtoken.so" ${BUILD_FOLDER}/libsovtoken_${TARGET_ARCH}/lib
    mv "${BUILD_FOLDER}/libsovtoken_${TARGET_ARCH}/lib/libsovtoken.so" "${BUILD_FOLDER}/libsovtoken_${TARGET_ARCH}/lib/libsovtoken_shared.so"
#    statically_link_dependencies_with_libindy
}

build(){
printenv
    echo "**************************************************"
    echo "Building for architecture ${ARCH} using ${TRIPLET}"
    echo "Toolchain path ${TOOLCHAIN_DIR}"
    echo "Sodium path ${SODIUM_DIR}"
    echo "Openssl path ${OPENSSL_DIR}"
    echo "Indy path ${INDY_DIR}"
    echo "Artifacts will be in ${LIBVCX_BUILDS}"
    echo "**************************************************"
    
    if [ ${BUILD_LIBINDY} -eq 1 ] ; then
        if [ ! -d "${INDY_DIR}" ] ; then
            echo "Cloning indy-sdk master branch"
            git clone --depth 1 https://github.com/hyperledger/indy-sdk.git ${INDY_DIR}
        fi
        pushd ${INDY_DIR}/libindy
        cargo build --release --target=${TRIPLET}
        popd
    fi
    export LIBINDY_DIR="${INDY_DIR}/libindy/target/${TRIPLET}/release"
    pushd ${WORKDIR}
#    export LIBINDY_STATIC=1
#        rm -rf target/${TRIPLET}
#        cargo clean
#        RUSTFLAGS="-L${TOOLCHAIN_DIR}/i686-linux-android/lib -lgnustl_shared" \
            cargo build --verbose --target=${TRIPLET}
    popd
}



#cleanup(){
##    rm -rf ${BUILD_FOLDER}
#
#}

#execute_build_steps(){
#
#        test
##        build
##        package_library
#}

generate_arch_flags ${TARGET_ARCH}
setup_dependencies
#download_and_unzip_dependencies_for_all_architectures
download_and_setup_toolchain
set_env_vars
create_standalone_toolchain_and_rust_target
create_cargo_config
build
#package_library
