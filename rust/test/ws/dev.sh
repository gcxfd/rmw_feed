#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

if ! hash watchexec 2>/dev/null; then
cargo install watchexec
fi

cd src

watchexec \
  -w . \
  -n -c -r -e coffee \
  -- ../run.sh ws
