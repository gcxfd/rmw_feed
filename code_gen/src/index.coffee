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
  write fp, cmd[..begin_pos] + gen(cmd[begin_pos+1...end_pos]) + cmd[end_pos..]

enum_name = (i)=>
  i.replace(/[<,>]/g,'')


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
      args = args.map((i)=>i.split(":").map((x)=>x.trim())).filter Boolean

      for i from args
        cls = i[1]
        i[1] = CLS_MAP[cls] or cls

      cmd = upperFirst(camelCase(name))
      api_cmd.push [cmd, args, rt]

  await Promise.all [
    modify(
      'wasm/src/reply.rs'
      'Err(err.into()),'
      '}'
      (txt)=>
        li = txt.split("(").map((i)=>i.split('::',2)[1]).filter(Boolean)
        exist = new Set(li)
        for i in api_cmd
          i = i[2]
          if i
            i = enum_name i
            if not exist.has(i)
              exist.add i
              txt += "  Reply::#{i}(r) => Ok(r.into()),\n  "
        txt
    )
    modify(
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
            t = x[1]
            if t.length
              args='('+t.map((x)=>'\n    '+x[1]+', //'+x[0]+'\n').join('')+'  )'
            else
              args = ''
            x[0]+args
        ).join(',\n  ')+',\n'
    )
    modify(
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
            i = enum_name(i)+'('+i+')'
            if not exist.has(i)
              rt_set.add i

        rt_set = [...rt_set]
        if rt_set.length
          cmd += '  '+rt_set.join(',\n  ')+',\n'

        cmd
    )
  ]
  return

if process.argv[1] == decodeURI (new URL(import.meta.url)).pathname
  await main()
  process.exit()
