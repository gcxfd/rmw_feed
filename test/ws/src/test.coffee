#!/usr/bin/env coffee

for i from [1..5]
  console.log i
  if i > 3
    throw new Error i
