<template>
  <div class="footer d-flex align-items-center justify-content-center">
    <div class="content"><h3>IMPORTANT</h3>
      <p class="align-self-center">This project is heavily inspired by <a href="https://www.shadertoy.com"
                                                                          target="_blank">ShaderToy</a>.
        I tried to rebuild some of
        ShaderToys functionality on my own, using TypeScript and WebAssembly.<br>
        Because i want to test this project on my IPhone, i will use WebGL 1 instead of WebGL2. But it is likely for me
        to rebuild this project later, using WebGPU (by utilizing wgpu-rs and WASM).
      </p></div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';

export default defineComponent({
  name: 'SdxFooter',
})
</script>

<style lang="scss" scoped>

.footer {
  height: 180px;
  background-color: #EEEEEE;
}

</style>
