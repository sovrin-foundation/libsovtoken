#!/bin/sh
set -xev

export PKG_CONFIG_ALLOW_CROSS=1
export CARGO_INCREMENTAL=1
export RUST_LOG=indy=trace
export RUST_TEST_THREADS=1
export INDY_VERSION=v1.16.0
export IOS_TARGETS="aarch64-apple-ios x86_64-apple-ios"
export WORK_DIR="${PWD}/../../../.macosbuild"
export LIBSOV_DIR="${PWD}/../../.."
export INDY_SDK_DIR=$WORK_DIR/sovtoken-indy-sdk
export LIBS_DIR=$WORK_DIR/libs
export OPENSSL_TARGETS="ios-sim-cross-x86_64 ios64-cross-arm64"

mkdir -p ${WORK_DIR}
mkdir -p ${LIBS_DIR}

prepare_openssl_dir() {
    OPENSSL_BASE_DIR=$(brew --cellar openssl)
    for f in $(ls -t "$OPENSSL_BASE_DIR"); do
      local ABSOLUTE_FILE_PATH="${OPENSSL_BASE_DIR}/${f}"
      if [ -d "$ABSOLUTE_FILE_PATH" ] && [ -d "$ABSOLUTE_FILE_PATH/lib" ]; then
        export OPENSSL_VERSION=$f
        export OPENSSL_DIR=$ABSOLUTE_FILE_PATH
        break
      fi
    done
    if [ -z "$OPENSSL_VERSION" ]; then
      echo >&2 "Error: Failed to find an OpenSSL installation in $OPENSSL_BASE_DIR"
      exit 1
    else
      echo "Found OpenSSL version $OPENSSL_VERSION"
    fi
}

extract_arch() {
    case $1 in
        aarch64-apple-ios) echo "arm64" ;;
        x86_64-apple-ios) echo "x86_64" ;;
        ios-sim-cross-x86_64) echo "x86_64" ;;
        ios64-cross-arm64) echo "arm64" ;;
        \?) exit 1
    esac
}

build_crypto() {
    if [ ! -d $WORK_DIR/OpenSSL-for-iPhone ]; then
        git clone https://github.com/x2on/OpenSSL-for-iPhone.git $WORK_DIR/OpenSSL-for-iPhone
    fi

    if [ ! -d $WORK_DIR/OpenSSL-for-iPhone/lib ]; then
        pushd $WORK_DIR/OpenSSL-for-iPhone
            ./build-libssl.sh --version=$OPENSSL_VERSION --targets="$OPENSSL_TARGETS" --verbose-on-error
            # export OPENSSL_LOCAL_CONFIG_DIR="$PWD/config"
        popd
    fi
    for TARGET in ${IOS_TARGETS[*]}; do
        pushd $WORK_DIR/OpenSSL-for-iPhone
            ARCH=$(extract_arch $TARGET)
            mkdir -p $LIBS_DIR/openssl/$ARCH/lib
            lipo -thin $ARCH -o $LIBS_DIR/openssl/$ARCH/lib/libssl.a ./lib/libssl.a
            lipo -thin $ARCH -o $LIBS_DIR/openssl/$ARCH/lib/libcrypto.a ./lib/libcrypto.a
            cp -r bin include $LIBS_DIR/openssl/$ARCH
        popd
    done
}

checkout_indy_sdk() {
    if [ ! -d $INDY_SDK_DIR ]; then
        git clone https://github.com/hyperledger/indy-sdk $INDY_SDK_DIR
    fi

    pushd $INDY_SDK_DIR
        git fetch --all
        git checkout $INDY_VERSION
    popd
}

build_libindy() {
    pushd $INDY_SDK_DIR/libindy
        for TARGET in ${IOS_TARGETS[*]}
        do
            ARCH=$(extract_arch $TARGET)
            mkdir -p $LIBS_DIR/indy/$ARCH

            export OPENSSL_DIR=$LIBS_DIR/openssl/$ARCH
            cargo lipo --release --targets=$TARGET

            cp -v $INDY_SDK_DIR/libindy/target/$TARGET/release/libindy.a $LIBS_DIR/indy/$ARCH/libindy.a
        done
    popd
}

copy_libindy_architectures() {
    for TARGET in ${IOS_TARGETS[*]}; do
        ARCH=$(extract_arch $TARGET)
        mkdir -p $LIBS_DIR/indy/$ARCH
        cp -v $INDY_SDK_DIR/libindy/target/$TARGET/release/libindy.a $LIBS_DIR/indy/$ARCH/libindy.a
    done
}

build_libsovtoken() {
    pushd $LIBSOV_DIR
        to_combine=""
        for TARGET in ${IOS_TARGETS[*]}
        do
            ARCH=$(extract_arch $TARGET)
            export LIBINDY_DIR=${LIBS_DIR}/indy/$ARCH
            export OPENSSL_DIR=$LIBS_DIR/openssl/$ARCH
            cargo lipo --release --verbose --targets="${TARGET}"

            mv ./target/$TARGET/release/libsovtoken.a ./target/$TARGET/libsovtoken-unstripped.a
            strip -S -x -o ./target/$TARGET/libsovtoken.a -r ./target/$TARGET/libsovtoken-unstripped.a

            to_combine="${to_combine} ./target/${TARGET}/libsovtoken.a"

            mkdir -p ./target/universal/release
            lipo -create $to_combine -o ./target/universal/release/libsovtoken.a
            cp ./target/universal/release/libsovtoken.a ./target/universal/libsovtoken-unstripped.a
            strip -S -x -o ./target/universal/libsovtoken.a -r ./target/universal/libsovtoken-unstripped.a

            BUILD_TIME=$(date -u "+%Y%m%d%H%M")
            GIT_REV=$(git rev-parse --short HEAD)
            LIBSOVTOKEN_VER=$(grep ^version Cargo.toml | head -n 1 | cut -d '"' -f 2)
            mv target libsovtoken
            zip -qq "libsovtoken_${LIBSOVTOKEN_VER}-${BUILD_TIME}-${GIT_REV}_all.zip" `find libsovtoken -type f -name "libsovtoken.a" | egrep '(ios|universal)' | egrep -v 'deps|debug|release'`
            mv libsovtoken target
        done
    popd
}

prepare_openssl_dir
build_crypto

checkout_indy_sdk
build_libindy
copy_libindy_architectures

build_libsovtoken
