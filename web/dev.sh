#!/usr/bin/env bash
set -e
DIR=$( dirname $(realpath "$0") )

cd $DIR

npx concurrently --kill-others \
  "npx coffee -wc vite"\
  "nr dev"
