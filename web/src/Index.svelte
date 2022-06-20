<script lang="coffee">
#import '~/styl/init.styl'
import ws from '~/coffee/ws.coffee'
import logo from ':/svg/logo.svg'
import Counter from './lib/Counter.svelte'

ws()

我 = 3333
x = 4
console.log {我}
:$ y = x + z

:$
  z = 我 + x - 2

:out
  for i from [1,2,3]
    for j from [4,5,6]
      console.log i,j
      if i>1
        break out

click = =>
  ++x
  return
</script>

<template lang="pug">
main
  img(alt="logo" src="{ logo }")
  button(on:click="{ click }")
    +if('x%2') if x%2
      +else else
  h2 x {x} y {y} z {z}
  Counter
</template>

<style lang="stylus">
button
  width 8rem
  font-size 32px

main
  justify-content center
  align-items center
  flex-direction column
  display flex

img
  height 16rem
  width 16rem
</style>
