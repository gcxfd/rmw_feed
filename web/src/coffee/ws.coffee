#!/usr/bin/env coffee

import WasmInit, {W,connect,ws} from ':/wasm/api/wasm.js'

await WasmInit()

RETRY = 1

export default WS = ws(
  "ws://127.0.0.1:4910"
  =>
    RETRY = 1
    return
  =>
    setTimeout(
      =>
        if RETRY < 99
          ++RETRY
        connect(WS)
        return
      RETRY*50
    )
    return
)

#console.log(await WS.stop())

###
WS = undefined
LI = []

_conn = (callback)=>
  if WS
    return WS


  ws = new WebSocket()
  ws.binaryType = "arraybuffer"

  ws.onmessage = ({data})=>
    if data instanceof ArrayBuffer
      console.log data
    else
      console.log "text",data

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
    callback?()
    return

  return

send = (msg)=>
  if WS
    WS.send(msg)
  else
    LI.push msg
  return

export default =>
  await Promise.all [
    WasmInit()
    new Promise (resolve)=>
      _conn(resolve)
      return
  ]
  send api.conf()
  return
###
