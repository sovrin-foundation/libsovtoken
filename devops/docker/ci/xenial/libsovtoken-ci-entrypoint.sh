#!/bin/bash
set -e

GEN_ARGS="--nodes 4 --clients 5 --nodeNum 1 2 3 4 --ips=$POOL_IP,$POOL_IP,$POOL_IP,$POOL_IP"

if [[ "$(id -u)" = '0' ]]; then
    gosu indy generate_indy_pool_transactions $GEN_ARGS
    gosu indy /usr/bin/supervisord &
else
    generate_indy_pool_transactions $GEN_ARGS
    /usr/bin/supervisord &
fi

exec "$@"
