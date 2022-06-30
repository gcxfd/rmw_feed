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
    console.log fn

  #for await line from fsline
  #  line = line.trim()
  #  t = []
  #  if line.startsWith("pub fn ")
  #    t.push line

  return

if process.argv[1] == decodeURI (new URL(import.meta.url)).pathname
  await main()
  process.exit()
