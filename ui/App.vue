<template>
  <SdxHeader
      :shader-list="shaderList"
      @compile="compile"
  />

  <SdxSurface
      :code="code"
  />

  <SdxEditor
      @compile="compile"
      :code="code"
      :shader-list="shaderList"
  />


  <!--  <div class="ide">editor</div>-->
  <!--  <SdxSurface-->
  <!--      :code="code"-->
  <!--  />-->

  <!--  <div class="right">-->
  <!--    <SdxEditor-->
  <!--        :code="code"-->
  <!--        @compile="compile"-->
  <!--    />-->
  <!--  </div>-->

  <SdxFooter/>

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
      code: shaders[2].code,
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
@import "../node_modules/bootstrap/scss/functions";
@import "../node_modules/bootstrap/scss/variables";
@import "../node_modules/bootstrap/scss/mixins";

body {
  background-color: #f8f9fa;
}

#app {
  display: grid;

  height: 100vh;
  width: 100vw;
  //grid-template: auto 1fr auto / 1fr 1fr;

  grid-template: auto 1fr 1fr auto / 1fr;

  header {
    grid-column: 1 / 2;
  }

  footer {
    grid-column: 1 / 2;
  }

  @include media-breakpoint-up(xl) {
    grid-template: auto auto 1fr / 1fr 1fr;

    header {
      grid-column: 1 / 3;
    }

    footer {
      grid-column: 1 / 3;
    }
  }
}

</style>