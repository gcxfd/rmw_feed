#!/usr/bin/env coffee

import WasmInit, * as api from ':/wasm/api/wasm.js'

WS = undefined
LI = []

_conn = =>
  if WS
    return WS

  resolve = undefined
  promise = new Promise (r)=>
    resolve = r
    return

  ws = new WebSocket("ws://127.0.0.1:4910")

  ws.onmessage = (msg)=>
    console.log await msg.data
    return

    return

  ws.onclose = =>
    WS = undefined
    setTimeout(_conn, 1000)
    return

  ws.onerror = (err)=>
    console.error "❌",err.error
    ws.close()
    return

  ws.onopen = =>
    #send get('127.0.0.1:3232','/1/2/3')
    #close()
    #send api.stop()
    loop
      msg = LI.shift()
      if msg == undefined
        break
      ws.send(msg)
    WS = ws
    resolve()
    return

  promise


send = (msg)=>
  if WS
    WS.send(msg)
  else
    LI.push msg
  return

export default =>
  await Promise.all [
    WasmInit()
    _conn()
  ]
  #send api.stop()
  return
