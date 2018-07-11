#!/bin/sh
set -e

/usr/bin/supervisord &

exec "$@"
