<script lang="coffee">
import { onMount } from 'svelte'

:$ input

:$ name = ''

:$ submit = =>
  input.focus()
  console.log name
  return

onMount =>
  input.focus()
  return

</script>

<template lang="pug">
form(on:submit|preventDefault="{ submit }")
  input(bind:this="{ input }" bind:value="{ name }" placeholder="请输入姓名")
  button(type="submit") 创建用户
</template>

<style lang="stylus" scoped></style>
