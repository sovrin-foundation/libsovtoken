#!/bin/sh

source ./shared.functions.sh

START_DIR=${PWD}
WORK_DIR=$(get_work_dir)
mkdir -p ${WORK_DIR}
SHA_HASH_DIR=$(abspath "${START_DIR}/../..")

source ./mac.02.libindy.env.sh

CLEAN_BUILD="cleanbuild"
if [ ! -z "$3" ]; then
    CLEAN_BUILD=$3
fi

if [ ! -d $WORK_DIR/sovtoken-indy-sdk ]; then
    git clone --depth 1 --single-branch -b rc https://github.com/hyperledger/indy-sdk.git $WORK_DIR/sovtoken-indy-sdk

fi
cd $WORK_DIR/sovtoken-indy-sdk

if [ "$CLEAN_BUILD" = "cleanbuild" ]; then
    git checkout .
    git checkout rc
    git clean -f
    git clean -fd
    git pull
    git checkout `cat $SHA_HASH_DIR/libindy.commit.sha1.hash.txt`
    #cd $WORK_DIR/vcx-indy-sdk
    #git checkout tags/v1.3.0
else
    git checkout -- libindy/Cargo.toml
    git checkout -- libnullpay/Cargo.toml
fi

DEBUG_SYMBOLS="debuginfo"
if [ ! -z "$1" ]; then
    DEBUG_SYMBOLS=$1
fi

IOS_TARGETS="aarch64-apple-ios,x86_64-apple-ios"
if [ ! -z "$2" ]; then
    IOS_TARGETS=$2
fi

#########################################################################################################################
# Now build libindy
#########################################################################################################################
cd ${WORK_DIR}/sovtoken-indy-sdk/libindy

if [ "$DEBUG_SYMBOLS" = "debuginfo" ]; then
    cat $START_DIR/cargo.toml.add.debug.txt >> Cargo.toml
fi

if [ "$CLEAN_BUILD" = "cleanbuild" ]; then
    cargo clean
    # cargo update
fi

cargo lipo --release --verbose --targets="${IOS_TARGETS}"

for arch in ${IOS_TARGETS[@]}; do
    if [ -f ./target.$arch/release/libindy.a ] ; then
        mv ./target/$arch/release/libindy.a ./target/$arch/libindy.a
    fi
done

if [ -f ./target/universal/release/libindy.a ] ; then
    cp ./target/universal/release/libindy.a ./target/universal/libindy.a
fi

BUILD_TIME=$(date -u "+%Y%m%d%H%M")
GIT_REV=$(git rev-parse --short HEAD)
LIBINDY_VER=$(grep ^version Cargo.toml | head -n 1 | cut -d '"' -f 2)
mv target libindy
zip "libindy_${LIBINDY_VER}-${BUILD_TIME}-${GIT_REV}_all.zip" `find libindy -type f -name "libindy.a" | egrep '(ios|universal)' | egrep -v 'deps|debug|release'`
mv libindy/"libindy_${LIBINDY_VER}-${BUILD_TIME}-${GIT_REV}_all.zip" .
mv libindy target
