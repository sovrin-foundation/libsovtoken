#!/bin/bash
set -ex

if [[ -n "$SKIP_ENTRY" ]]; then
    exec "$@"
fi

archs=${TARGET_ARCHS1:-"arm arm64 x86"}
api=${TARGET_API:-21}
android_ndk_base_dir=${ANDROID_NDK_BASE_DIR:-"/tmp/android-ndk"}

PATH_ORIG="$PATH"

android-ndk-install "$archs" "$api" "$android_ndk_base_dir"

set_env() {
    local arch="$1"
    local prebuilt_dir="$PREBUILT_DIR$arch"

    case $arch in
        arm) CROSS_COMPILE="arm-linux-androideabi" ;;
        arm64) CROSS_COMPILE="aarch64-linux-android" ;;
        x86) CROSS_COMPILE="i686-linux-android" ;;
        \?) echo STDERR "Unknown TARGET_ARCH"
            exit 1
            ;;
    esac

    export CROSS_COMPILE
    export PKG_CONFIG_ALLOW_CROSS=1
    export TARGET=android

    find $PREBUILT_DIR -name "*$arch.zip" \
        -exec mkdir -p "$prebuilt_dir" \; \
        -exec echo Unzipping '{}' \; \
        -exec unzip -q '{}' -d "$prebuilt_dir" \;

    export TOOLCHAIN_DIR="$android_ndk_base_dir/$api/$arch"
    export CC="${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang"
    export AR="${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ar"
    export CXX="${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang++"
    export CXXLD="${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ld"
    export RANLIB="${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ranlib"
    export PATH="${BUILD_DIR}/.cargo/bin:${TOOLCHAIN_DIR}/bin:${PATH_ORIG}"

    export OPENSSL_DIR="$prebuilt_dir/openssl_${arch}"
    export SODIUM_LIB_DIR="$prebuilt_dir/libsodium_${arch}/lib"
    export SODIUM_INCLUDE_DIR="$prebuilt_dir/libsodium_${arch}/include"
    export LIBZMQ_LIB_DIR="$prebuilt_dir/libzmq_${arch}/lib"
    export LIBZMQ_INCLUDE_DIR="$prebuilt_dir/libzmq_${arch}/include"

    pushd "$BUILD_DIR"
    mkdir -p .cargo
    echo "[target.${CROSS_COMPILE}]" > .cargo/config
    echo "ar = \"${AR}\"" >> .cargo/config
    echo "linker = \"${CC}\"" >> .cargo/config
    rustup target add ${CROSS_COMPILE}
    rustup target list
    popd
}

for arch in $archs; do
    set_env $arch

    pushd $INDY_SDK_DIR/libindy
    cargo build --release --target="${CROSS_COMPILE}"
    echo "libindy android build successful for arch $arch, api $api"
    popd
done

export PATH="$PATH_ORIG"

echo "Done"

exec "$@"
