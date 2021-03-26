<template>
  <header>
    <SdxHeader
      :shader-list="shaderList"
      @compile="compile"
    />
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
import shaders from './shaders';

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
      code: shaders[0].code,
      shaderList: shaders,
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