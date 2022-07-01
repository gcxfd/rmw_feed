<script lang="coffee">
import { onMount } from 'svelte'

+ input

name = ''

submit = =>
  input.focus()
  console.log name
  return

onMount =>
  input.focus()
  return

</script>

<template lang="pug">
form(@submit|preventDefault=submit)
  input(:this=input :value=name placeholder="请输入姓名")
  button(type="submit") 创建用户
</template>

<style lang="stylus" scoped></style>
