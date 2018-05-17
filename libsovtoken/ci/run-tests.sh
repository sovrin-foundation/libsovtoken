#!/bin/bash

RUST_DIR=${1:-..}
DOCKERFILE=${2:-ubuntu.dockerfile}
DOCKERIMAGE=${3:-libsovtoken}
DOCKER_IMAGE_ID=$(docker image ls | grep ${DOCKERIMAGE} | perl -pe 's/\s+/ /g' | cut -d ' ' -f 3)

echo "Building libsovtoken in ${PWD}/${RUST_DIR}"

if [ -z "${DOCKER_IMAGE_ID}" ] ; then
    echo "Docker image will be built"
    docker build -f ${DOCKERFILE} -t ${DOCKERIMAGE} ${PWD}/$RUST_DIR/ci
else
    echo "Using existing docker image"
fi

docker run --rm -w /data -v "${PWD}/${RUST_DIR}:/data" -t ${DOCKERIMAGE} cargo test --color=always -- --nocapture
