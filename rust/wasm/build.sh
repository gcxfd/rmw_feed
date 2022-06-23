#!/usr/bin/env bash

DIR=$(cd "$(dirname "$0")"; pwd)
cd $DIR
set -ex

if ! [ -x "$(command -v wasm-bindgen)" ]; then
cargo install -f wasm-bindgen-cli
fi

if ! [ -x "$(command -v wasm-opt)" ]; then
npm install -g wasm-opt
fi

cargo build --target wasm32-unknown-unknown --release

NAME=wasm

rust_wasm=../target/wasm32-unknown-unknown/release/$NAME.wasm

wasm-bindgen \
  $rust_wasm \
  --out-dir pkg \
  --target web \
  --weak-refs \
  --reference-types

wasm-opt --enable-reference-types -O3 -o ./pkg/api_bg.wasm ./pkg/api_bg.wasm
rsync -av ./pkg/* ../../web/file/wasm/api/
