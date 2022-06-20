#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex
export WEBDIR=$DIR
bash -c "sleep 1.5 && open http://127.0.0.1:9999"
exec openresty -c $DIR/nginx.conf
