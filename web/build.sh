#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

./sh/init.sh

nr build

#./sh/filename_min.coffee
