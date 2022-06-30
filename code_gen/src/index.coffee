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


modify = (fp, start, end, gen)=>
  cmd = await read fp
  begin_pos = cmd.indexOf(start)+start.length
  end_pos = cmd.indexOf(end,begin_pos)
  write fp, cmd[..begin_pos] + gen(cmd[begin_pos...end_pos]) + cmd[end_pos..]


export default main = =>
  api = await read 'net/src/api/cmd.rs'

  api_cmd = []

  for fn from extract_li api, "pub fn ","{"
    pos = fn.indexOf('(')
    if pos > 0
      name = fn[...pos]
      has_return = fn.lastIndexOf('->')
      if has_return > 0
        rt = fn[has_return+2..].trim()

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
      api_cmd.push [cmd, args, rt]

  await modify(
    'api/src/cmd.rs'
    'Stop,'
    '}'
    (cmd)=>
      exist = cmd.split(',').map(
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


      '  '+api_cmd.sort(
        (a,b)=>
          cmd_pos[a[0]] - cmd_pos[b[0]]
      ).map(
        (x)=>
          x[0]+x[1]
      ).join(',\n  ')+',\n'
  )

  await modify(
    'api/src/reply.rs'
    'None,'
    '}'
    (cmd)=>
      exist = new Set(cmd.split(',').map(
        (i)=>
          i.trim()
      ).filter(Boolean))
      rt_set = new Set()

      li = []
      for i in api_cmd
        i = i[2]
        if i
          i = i.replace(/[<,>]/g,'')+'('+i+')'
          if not exist.has(i)
            rt_set.add i

      cmd
  )

  return

if process.argv[1] == decodeURI (new URL(import.meta.url)).pathname
  await main()
  process.exit()
