#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

./build.sh
#./sh/qiniu.unload.coffee
