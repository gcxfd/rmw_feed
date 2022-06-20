#!/usr/bin/env coffee
import thisdir from '@rmw/thisdir'
import baseX from 'base-x'
import {dirname,join} from 'path'
import {rename,writeFile,readFile,opendir} from 'fs/promises'

BFILE = baseX '!$-.0123456789_abcdefghijklmnopqrstuvwxyz'

encode = (n)=>
  bin = Buffer.allocUnsafe 6
  bin.writeUIntBE(n,0,6)
  for i,pos in bin
    if i!=0
      break
  BFILE.encode bin[pos..]

ROOT = dirname thisdir import.meta
DIR = join ROOT, 'dist'
css_js = new Map()
to_replace = []
all = new Set()

IGNORE = new Set()

PUBLIC = join(
  ROOT
  'public'
)

for await fp from await opendir PUBLIC
  if not fp.isFile()
    continue
  IGNORE.add fp.name

for await fp from await opendir DIR
  if not fp.isFile()
    continue
  fp = fp.name
  if IGNORE.has fp
    continue
  all.add fp
  name = fp.split('.')
  ext = name.at -1
  hex = name.at -2
  if ['htm','html','css','js'].includes(ext)
    css_js.set(
      fp
      await readFile(join(DIR,fp),'utf8')
    )
  if not ['m.js','index.html','index.htm'].includes(fp)
    to_replace.push fp

ID = new Map()

fp_name = new Map()

for i from to_replace
  ext = i[i.lastIndexOf('.')+1..]
  if ext == 'html'
    ext = 'htm'
  id = ID.get(ext) or (
    1 #if ['node','svg'].includes(ext) then 1 else 0
  )
  loop
    name = encode(id++)+'.'+ext
    if not all.has name
      break
  ID.set(ext,id)
  fp_name.set(i,name)
  await rename(
    join(DIR,i)
    join(DIR,name)
  )

for [k,v] from css_js.entries()
  for [f,t] from fp_name.entries()
    t = t.replaceAll '$','$$$$'
    v = v.replaceAll(f,t)

  await writeFile(
    join DIR,(fp_name.get(k) or k)
    v
  )

process.exit()

#console.log css_js
