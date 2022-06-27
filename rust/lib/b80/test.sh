#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex
RUST_BACKTRACE=1 cargo +nightly watch -cx "test -- --nocapture"
