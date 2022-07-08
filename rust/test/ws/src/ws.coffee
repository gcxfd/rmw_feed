#!/usr/bin/env coffee


import WasmInit, {W,connect,ws} from ':/wasm/api/wasm.js'

await WasmInit()

RETRY = 2

export default WS = ws(
  "ws://127.0.0.1:4910"
  =>
    RETRY = 2
    return
  =>
    setTimeout(
      =>
        if RETRY < 99
          ++RETRY
        connect(WS)
        return
      RETRY*99
    )
    return
)
