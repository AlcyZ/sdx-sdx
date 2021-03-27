<template>
  <footer class="bg-light d-flex align-items-center">
    <div class="container my-3">
      <figure class="text-center">
        <blockquote class="blockquote">
          <p>Important:</p>
        </blockquote>
        <figcaption class="blockquote-footer">
          I tried to recreate some of the <a class="text-decoration-none" href="https://www.shadertoy.com"
                                             target="_blank">ShaderToys</a>
          functionality myself, using TypeScript and WebAssembly. Since I
          want to test this project on my IPhone, I will use WebGL1 instead of WebGL2. But it is likely that I will
          rebuild this project later with WebGPU <cite>(using wgpu-rs and WASM)</cite>.
        </figcaption>
      </figure>
    </div>
  </footer>
</template>

<script lang="ts">
import {defineComponent} from 'vue';

export default defineComponent({
  name: 'SdxFooter',
})
</script>
