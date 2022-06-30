#!/usr/bin/env bash

. $HOME/.asdf/asdf.sh

DIR=$(cd "$(dirname "$0")"; pwd)
set -ex

cd $DIR

if [ ! -f lib/index.js ]; then
pnpm run prepare
fi

cd lib

node="node --trace-warnings --es-module-specifier-resolution=node --trace-uncaught --expose-gc --unhandled-rejections=strict"

NODE_ENV=production exec $node index.js
