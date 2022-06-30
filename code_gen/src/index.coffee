#!/usr/bin/env coffee

#import fsline from '@rmw/fsline'
import thisdir from '@rmw/thisdir'
import {resolve,join,dirname} from 'path'
import {readFile,writeFile} from 'fs/promises'
import {extract_li} from './extract'
import {upperFirst, camelCase} from 'lodash-es'

PWD = thisdir(import.meta)
ROOT = dirname dirname PWD
RUST = join(ROOT,'rust')

UTF8 = 'utf8'

CLS_MAP = {
  'impl AsRef<str>':'String'
}

read = (fp)=>
  readFile join(RUST,fp), UTF8

write = (fp, txt)=>
  writeFile join(RUST,fp), txt



export default main = =>
  api = await read 'db/src/api.rs'

  api_cmd = []

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

      cmd = upperFirst(camelCase(name))
      t = []
      for [name,cls] from args
        if name && cls
          t.push(CLS_MAP[cls] or cls)

      if t.length
        args='('+t.join(',')+')'
      else
        args = ''
      api_cmd.push [cmd, args]


  src = 'api/src/cmd.rs'
  cmd = await read src

  stop = 'Stop,'

  begin_pos = cmd.indexOf(stop)+stop.length
  end_pos = cmd.indexOf('}',begin_pos)


  exist = cmd[begin_pos...end_pos].split(',').map(
    (i)=>
      i.split('(',1)[0].trim()
  ).filter(Boolean)

  len = exist.length
  cmd_pos = {}
  for [key] from api_cmd
    pos = exist.indexOf(key)
    if pos < 0
      pos = len
    cmd_pos[key]=pos


  api_cmd = '  '+api_cmd.sort(
    (a,b)=>
      cmd_pos[a[0]] - cmd_pos[b[0]]
  ).map(
    (x)=>
      x[0]+x[1]
  ).join(',\n  ')+'\n'

  await write src, cmd[..begin_pos] + api_cmd + cmd[end_pos..]

  #for await line from fsline
  #  line = line.trim()
  #  t = []
  #  if line.startsWith("pub fn ")
  #    t.push line

  return

if process.argv[1] == decodeURI (new URL(import.meta.url)).pathname
  await main()
  process.exit()
