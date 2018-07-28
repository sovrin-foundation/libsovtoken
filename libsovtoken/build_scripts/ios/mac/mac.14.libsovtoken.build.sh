#!/bin/sh

source ./shared.functions.sh

START_DIR=${PWD}
WORK_DIR=$(get_work_dir)
mkdir -p ${WORK_DIR}

source ./mac.02.libindy.env.sh
if [ ! -d "${WORK_DIR}/sovtoken-indy-sdk" ]; then
    echo STDERR "Unable to find ${WORK_DIR}/sovtoken-indy-sdk directory"
    exit 1
fi

IOS_TARGETS="aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,i386-apple-ios,x86_64-apple-ios"
if [ ! -z "$2" ]; then
    IOS_TARGETS=$2
fi

#########################################################################################################################
# Now build libsovtoken
#########################################################################################################################
cd $(get_sovtoken_dir)

if [ "$DEBUG_SYMBOLS" = "debuginfo" ]; then
    cat $START_DIR/cargo.toml.add.debug.txt >> Cargo.toml
fi

bkpIFS="$IFS"
IFS=',()][' read -r -a targets <<<"${IOS_TARGETS}"
echo "Building targets: ${targets[@]}"    ##Or printf "%s\n" ${array[@]}
IFS="$bkpIFS"

to_combine=""
for target in ${targets[*]}
do
    #if [ "${target}" = "aarch64-apple-ios" ]; then
    #    target_arch="arm64"
    #elif [ "${target}" = "armv7-apple-ios" ]; then
    #    target_arch="armv7"
    #elif [ "${target}" = "armv7s-apple-ios" ]; then
    #    target_arch="armv7s"
    #elif [ "${target}" = "i386-apple-ios" ]; then
    #    target_arch="i386"
    #elif [ "${target}" = "x86_64-apple-ios" ]; then
    #    target_arch="x86_64"
    #fi

    #export OPENSSL_LIB_DIR=$WORK_DIR/OpenSSL-for-iPhone/lib/${target_arch}
    #export IOS_SODIUM_LIB=$WORK_DIR/libzmq-ios/libsodium-ios/dist/ios/lib/${target_arch}
    #export IOS_ZMQ_LIB=$WORK_DIR/libzmq-ios/dist/ios/lib/${target_arch}
    export LIBINDY_DIR=${WORK_DIR}/sovtoken-indy-sdk/libindy/target/${target}/release
    #export LIBINDY_DIR=$WORK_DIR/vcx-indy-sdk/libindy/target/${target}/release
    cargo lipo --release --verbose --targets="${target}"

    to_combine="${to_combine} ./target/${target}/release/libsovtoken.a"

done
mkdir -p ./target/universal/release
lipo -create $to_combine -o ./target/universal/release/libsovtoken.a

for arch in ${IOS_TARGETS[@]}; do
    if [ -f ./target.$arch/release/libsovtoken.a ] ; then
        mv ./target/$arch/release/libsovtoken.a ./target/$arch/libsovtoken.a
    fi
done

if [ -f ./target/universal/release/libsovtoken.a ] ; then
    cp ./target/universal/release/libsovtoken.a ./target/universal/libsovtoken.a
fi

BUILD_TIME=$(date -u "+%Y%m%d%H%M")
GIT_REV=$(git rev-parse --short HEAD)
LIBSOVTOKEN_VER=$(grep ^version Cargo.toml | head -n 1 | cut -d '"' -f 2)
mv target libsovtoken
zip -qq "libsovtoken_${LIBSOVTOKEN_VER}-${BUILD_TIME}-${GIT_REV}_all.zip" `find libsovtoken -type f -name "libsovtoken.a" | egrep '(ios|universal)' | egrep -v 'deps|debug|release'`
mv libsovtoken target
