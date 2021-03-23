<template>
  <header>
    <SdxHeader/>
  </header>

  <SdxSurface
      :code="code"
  />

  <div class="right">
    <SdxEditor
        :start-value="code"
        @compile="compile"
    />
  </div>

  <footer>
    <SdxFooter/>
  </footer>
</template>

<script lang="ts">
import {defineComponent} from 'vue';
import SdxHeader from './Layout/SdxHeader.vue';
import SdxFooter from './Layout/SdxFooter.vue';
import SdxSurface from './Layout/SdxSurface.vue';
import SdxEditor from './Layout/SdxEditor.vue';

const stFs = `#version 100

precision mediump float;

uniform float iTime;
uniform vec3 iResolution;

void main() {
  vec2 uv =  gl_FragCoord.xy/iResolution.xy;

  vec3 col = 0.5 + 0.5 * cos(iTime + uv.xyx + vec3(0, 2, 4));

  gl_FragColor = vec4(col, 1.0);
}`;

const startFs = `#version 100
precision mediump float;
uniform float iTime;

void main() {
    gl_FragColor = vec4(1.0 , 0.0, 0.0, 1.0);

    // float sinT = sin(iTime);
    // float red = (sinT + 1.0) * 0.5;
    // float green = fract(sinT);
    //
    // gl_FragColor = vec4(red, green, 0.0, 1.0);
}`;

export default defineComponent({
  name: 'App',
  components: {
    SdxHeader,
    SdxFooter,
    SdxSurface,
    SdxEditor,
  },

  data() {
    return {
      code: stFs
    }
  },

  methods: {
    compile(code: string): void {
      this.code = code;
    }
  }
})
</script>

<style lang="scss">
#app {
  display: grid;
  grid-template: auto 1fr auto / 1fr 1fr;
  height: 100vh;
  width: 100vw;

  header {
    grid-column: 1 / 3;
  }

  footer {
    grid-column: 1 / 3;
  }
}
</style>