#!/usr/bin/env coffee

import * as wasm from ':/wasm/api/wasm.js'
import conn from './conn.coffee'

export default await conn(wasm)
