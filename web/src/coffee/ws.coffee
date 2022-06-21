#!/usr/bin/env coffee

import WasmInit, * as api from ':/wasm/api/wasm.js'


export default =>
  wasmInit = WasmInit()

  ws = new WebSocket("ws://127.0.0.1:4910")

  ws.onmessage = (msg)=>
    console.log await msg.data.arrayBuffer()
    return

  send = (msg)=>
    console.log msg
    ws.send(msg)
    return
#
#ws.onerror = (err)=>
#  console.error "❌",err.error
#  return
#
  ws.onclose = =>
    console.log "ws open"
    return

  ws.onopen = =>
    #send get('127.0.0.1:3232','/1/2/3')
    #close()
    await wasmInit
    send api.stop()
    return
