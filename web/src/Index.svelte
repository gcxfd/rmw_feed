<script lang="coffee">
#import '~/styl/init.styl'
import src from ':/svg/logo.svg'
import Counter from './lib/Counter.svelte'
import UserNew from './lib/UserNew.svelte'
import ws from '~/coffee/ws.coffee'

window.ws = ws
user_name = ws.user_name()

xxx = '33'

x = 4

:$
  z = 1 + x

:$ y = 2 + z

:$ enable = x%2


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
  +await user_name
    h1 loading
    +then name
      +if name == undefined
        UserNew(:xxx)
        +else
          h1 {name}
    +catch err
      h2 err
  button(@click=click)
    +if x%2
      | if x%2
      +else
        h3 else
  h2(class:red=enable) x {x} y {y} z {z}
  Counter
  #map
    img(:src alt="logo")
</template>

<style lang="stylus">
#map
  width 100%
  height 300px

.red
  color #f00

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
