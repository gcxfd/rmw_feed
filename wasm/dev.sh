#!/usr/bin/env bash

_DIR=$(dirname $(realpath "$0"))

cd $_DIR/..

./dev.sh wasm
