#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

if [ ! -n "$1" ] ;then
file=ws
else
file=${@:1}
fi

npx coffee -M -o $DIR/lib -c $DIR/src
deno run -A $DIR/lib/$file.js
