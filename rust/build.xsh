#!/usr/bin/env xonsh

from fire import Fire
from os.path import dirname,abspath,exists
import platform

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
  --release \
  -Z build-std=std,panic_abort \
  -Z build-std-features=panic_immediate_abort \
  -p @(app) \
  --target @(TARGET)

  out=f"target/{TARGET}/release/{app}"
  strip @(out)

#./sh/upx.sh
  upx --best --lzma @(out)

  print(out)
