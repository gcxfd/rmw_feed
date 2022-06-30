#!/usr/bin/env coffee

#import fsline from '@rmw/fsline'
import thisdir from '@rmw/thisdir'
import {resolve,join,dirname} from 'path'
import {readFile} from 'fs/promises'
import {extract_li} from './extract'

PWD = thisdir(import.meta)
ROOT = dirname dirname PWD
RUST = join(ROOT,'rust')

UTF8 = 'utf8'

export default main = =>
  api = await readFile join(RUST,'db/src/api.rs'), UTF8
  for fn from extract_li api, "pub fn ","{"
    pos = fn.indexOf('(')
    if pos > 0
      name = fn[...pos]
      has_return = fn.lastIndexOf('->')
      if has_return > 0
        rt = fn[has_return+2..]
      args = fn[pos+1...fn.lastIndexOf(')')].split(",")
      args.shift()
      args = args.map((i)=>i.split(":").map((x)=>x.trim()))
      console.log name, args,rt

  #for await line from fsline
  #  line = line.trim()
  #  t = []
  #  if line.startsWith("pub fn ")
  #    t.push line

  return

if process.argv[1] == decodeURI (new URL(import.meta.url)).pathname
  await main()
  process.exit()
