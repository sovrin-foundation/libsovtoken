#!/bin/sh

set -xv

function extract_target() {
    case $1 in
        aarch64-apple-ios) echo "arm64" ;;
        armv7-apple-ios) echo "armv7" ;;
        armv7s-apple-ios) echo "armv7s" ;;
        i386-apple-ios) echo "i386" ;;
        x86_64-apple-ios) echo "x86_64" ;;
        \?) exit 1
    esac
}

source ./shared.functions.sh

START_DIR=${PWD}
WORK_DIR=$(get_work_dir)
mkdir -p ${WORK_DIR}

source ./mac.02.libindy.env.sh


mkdir -p ${WORK_DIR}/sovtoken-indy-sdk
if [ ! -f "${WORK_DIR}/sovtoken-indy-sdk/libindy.a" ] ; then
    command pushd ${WORK_DIR}/sovtoken-indy-sdk > /dev/null
    curl -sSLO ${LIBINDY_IOS_BUILD_URL}
    tar -xf ${LIBINDY_FILE}
    command popd > /dev/null
    fi

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
echo $(get_sovtoken_dir)
cd $(get_sovtoken_dir)

if [ "$DEBUG_SYMBOLS" = "debuginfo" ]; then
    cat $START_DIR/cargo.toml.add.debug.txt >> Cargo.toml
fi

bkpIFS="$IFS"
IFS=',()][' read -r -a targets <<<"${IOS_TARGETS}"
echo "Building targets: ${targets[@]}"
IFS="$bkpIFS"


to_combine=""
for target in ${targets[*]}
do
    LIBINDY=${WORK_DIR}/sovtoken-indy-sdk

    export LIBINDY_DIR=${LIBINDY}/${target}
    mkdir -p ${LIBINDY_DIR}
    etarget=$(extract_target $target)

    echo "LIBINDY_DIR=${LIBINDY_DIR}"
    lipo -extract_family $etarget $LIBINDY/libindy.a -o $LIBINDY_DIR/libindy.a
    cargo lipo --release --verbose --targets="${target}"
    mv ./target/$target/release/libsovtoken.a ./target/$target/libsovtoken-unstripped.a
    strip -S -x -o ./target/$target/libsovtoken.a -r ./target/$target/libsovtoken-unstripped.a

    to_combine="${to_combine} ./target/${target}/release/libsovtoken.a"
done
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
