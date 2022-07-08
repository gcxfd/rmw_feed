#!/usr/bin/env coffee

import * as wasm from './wasm/api/wasm.js'
import conn from './conn.js'

ws = await conn wasm

console.log await ws.user_name()
