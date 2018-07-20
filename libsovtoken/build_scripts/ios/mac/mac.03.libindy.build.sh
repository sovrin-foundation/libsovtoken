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
    git clone --depth 1 https://github.com/hyperledger/indy-sdk.git $WORK_DIR/sovtoken-indy-sdk

fi
cd $WORK_DIR/sovtoken-indy-sdk

if [ "$CLEAN_BUILD" = "cleanbuild" ]; then
    git checkout .
    git checkout master
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

IOS_TARGETS="aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,i386-apple-ios,x86_64-apple-ios"
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
