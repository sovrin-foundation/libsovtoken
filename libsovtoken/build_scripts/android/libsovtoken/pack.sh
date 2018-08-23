#!/usr/bin/env bash

set -ex

TARGET_DIR=$1
BUILD_TIME=${BUILD_TIME:-$(date -u "+%Y%m%d%H%M")}
GIT_REV=${GIT_REV:-$(git rev-parse --short HEAD)}
LIBSOVTOKEN_VER=${LIBSOVTOKEN_VER:-$(grep ^version ${LIBSOVTOKEN_DIR}/Cargo.toml | head -n 1 | cut -d '"' -f 2)}

command pushd $(dirname ${TARGET_DIR})
zip -r "libsovtoken_${LIBSOVTOKEN_VER}-${BUILD_TIME}-${GIT_REV}_all.zip" $(basename ${TARGET_DIR})
command popd
