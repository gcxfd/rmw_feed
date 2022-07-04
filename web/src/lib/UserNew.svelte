<script lang="coffee">
import { onMount } from 'svelte'
import {trim} from 'lodash-es'

+ input

< xxx

name = ''

submit = =>
  input.focus()
  console.log name.trim()
  return

onMount =>
  input.focus()
  return

</script>

<template lang="pug">
form(@submit|preventDefault)
  input(&input :value=name placeholder="请输入姓名")
  button(type="submit") 创建用户 {xxx}
</template>

<style lang="stylus" scoped></style>
