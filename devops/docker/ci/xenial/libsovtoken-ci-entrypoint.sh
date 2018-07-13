#!/bin/bash
set -e

GEN_ARGS="--nodes 4 --clients 5 --nodeNum 1 2 3 4 --ips=$POOL_IP,$POOL_IP,$POOL_IP,$POOL_IP"

if [ -n "${INDY_POOL_LOG_LEVEL}" ]; then
    echo -ne "\nlogLevel=${INDY_POOL_LOG_LEVEL}\n" >> /etc/indy/indy_config.py
fi

if [[ "$(id -u)" = '0' ]]; then
    gosu indy generate_indy_pool_transactions $GEN_ARGS
    gosu indy /usr/bin/supervisord
else
    generate_indy_pool_transactions $GEN_ARGS
    /usr/bin/supervisord
fi

# TODO improve the logic, options:
#   - parse nodes' validator info
#   - use client to connect to pool
#   - check service status using supervisor
#   - ...
try=1
timeout=30
echo 'Wainting for pool is started...' >&2
until ls /var/log/indy/sandbox/Node1.log >/dev/null 2>&1
do
	if [ "$try" -gt "$timeout" ]; then
		echo -e "\nERROR: Pool seems not started after $timeout seconds." >&2
        ps axf >&2
        cat /tmp/node1.log >&2 || true
		exit 1
	fi
    echo -n '.' >&2
    try=$(( $try + 1 ))
	sleep 1
done
echo -e "\nPool is started." >&2

exec "$@"
