#!/usr/bin/env coffee


#import WasmInit,{get} from '../../wasm/pkg/wasm.js'
#
#await WasmInit()

export default =>

  ws = new WebSocket("ws://127.0.0.1:4910")

#ws.onmessage = (msg)=>
#  return
#
#send = (msg)=>
#  console.log msg
#  ws.send(msg)
#  return
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
    console.log "ws open"
    return
