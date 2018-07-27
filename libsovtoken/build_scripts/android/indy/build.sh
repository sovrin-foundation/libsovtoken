#!/bin/bash

LIBSOVTOKEN_DIR=$(perl -e 'use Cwd "abs_path"; print abs_path(shift)' "${PWD}/../../../..")
DOCKER_IMAGE_ID=$(docker image ls | grep libindy-android)

if [ -z "${DOCKER_IMAGE_ID}" ] ; then
    docker build -t libindy-android:latest .
fi

docker run --rm -v ${LIBSOVTOKEN_DIR}:/data -w /data/libsovtoken/build_scripts/android/indy -t libindy-android:latest /bin/bash build.nondocker.sh "$@"
