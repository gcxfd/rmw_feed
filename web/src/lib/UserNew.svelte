<script lang="coffee">
:$ name = ''
</script>

<template lang="pug">
form
  input(bind:value="{ name }" placeholder="请输入姓名")
  button(type="submit") 创建用户
</template>

<style lang="stylus" scoped></style>
