#!/usr/bin/env coffee

ws = new WebSocket("ws://127.0.0.1:4910")

ws.onmessage = (msg)=>
  console.log msg.data
  return

ws.onerror = (err)=>
  return

ws.onopen = =>
  console.log 'open'
  return
