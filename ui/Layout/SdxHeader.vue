<template>
  <div class="header d-flex">
    <div class="item brand mx-4">Sdx Shader</div>
    <div class="item flex-grow-1">
      <span>Educational project - Heavily inspired by  <a href="https://www.shadertoy.com" target="_blank">ShaderToy</a></span>
    </div>
    <div class="item mx-4">
      <a href="#">New</a>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';

export default defineComponent({
  name: 'SdxHeader',
})
</script>

<style lang="scss" scoped>
.header {
  height: 70px;
  background-color: #eeeeee;

  .item {
    display: flex;
    align-items: center;
    justify-content: center;

    a {
      text-decoration: none;
    }
  }
}
</style>