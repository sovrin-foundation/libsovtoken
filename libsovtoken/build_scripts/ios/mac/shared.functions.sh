#!/bin/sh

export LIBINDY_IOS_BUILD_URL="https://repo.sovrin.org/ios/libindy/stable/libindy-core/1.16.0/libindy.tar.gz"
export LIBINDY_FILE=$(basename ${LIBINDY_IOS_BUILD_URL})
export LIBINDY_VERSION=$(basename $(dirname ${LIBINDY_IOS_BUILD_URL}))
export BUILD_CACHE=~/.build_libvxc/ioscache

function abspath() {
    perl -e 'use Cwd "abs_path"; print abs_path(shift)' $1
}

function get_sovtoken_dir() {
    TMP=$(abspath "${PWD}/../../..")
    echo ${TMP}
}
function get_work_dir() {
    TMP=$(get_sovtoken_dir)
    TMP="${TMP}/.macosbuild"
    echo ${TMP}
}
