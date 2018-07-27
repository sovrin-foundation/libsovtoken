#!/bin/bash

LIBSOVTOKEN_DIR=$(perl -e 'use Cwd "abs_path"; print abs_path(shift)' "${PWD}/../../../..")
DOCKER_IMAGE_ID=$(docker image ls | grep libsovtoken-android)

if [ -z "${DOCKER_IMAGE_ID}" ] ; then
    docker build -t libsovtoken-android:latest .
fi

docker run --rm -v ${LIBSOVTOKEN_DIR}:/data -w /data/libsovtoken/build_scripts/android/libsovtoken -t libsovtoken-android:latest /bin/bash build.nondocker.sh "$@"
