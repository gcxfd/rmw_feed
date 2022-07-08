#!/usr/bin/env xonsh

from fire import Fire
from os.path import dirname,abspath,exists
import platform
from humanize import naturalsize
import os

PWD = dirname(abspath(__file__))

cd @(PWD)

p".xonshrc".exists() && source .xonshrc

cd @(PWD)

system = platform.system().lower()
if system == 'darwin':
  system = f'apple-{system}'
elif system == 'linux':
  system = 'unknown-linux-gnu'
# $RUSTFLAGS="-C target-feature=+crt-static -C link-self-contained=yes -L native=/usr/lib -l static=clang"
  # -l static=stdc++"

# x86_64-unknown-linux-gnu
# system = 'unknown-linux-gnu'

TARGET=f'{platform.machine()}-{system}'

@Fire
def main(app="rmw"):
  cargo build \
  -p @(app) \
  --release \
  --target @(TARGET) \
  -Z build-std=std,panic_abort

# -Z build-std-features=panic_immediate_abort

  out=f"target/{TARGET}/release/{app}"
  strip @(out)

#./sh/upx.sh
  upx --best --lzma @(out)

  stat = os.stat(out)
  print(naturalsize(stat.st_size))
  print(out)
