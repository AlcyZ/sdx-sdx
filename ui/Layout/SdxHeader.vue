<template>
  <div class="header d-flex">
    <div class="item brand mx-4">Sdx Shader</div>
    <div class="item flex-grow-1">
      <span>Educational project - Heavily inspired by  <a href="https://www.shadertoy.com" target="_blank">ShaderToy</a></span>
    </div>
    <div class="item">
      <div class="dropdown">
        <button class="btn btn-secondary dropdown-toggle" type="button" id="dropdownMenuButton1"
                data-bs-toggle="dropdown" aria-expanded="false">
          {{ selected?.name || 'Shaders' }}
        </button>
        <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
          <li v-for="(shaderItem, i) in shaderList" :key="i">
            <a @click="selectShader(shaderItem)" class="dropdown-item" href="#" v-text="shaderItem.name"></a>
          </li>
        </ul>
      </div>
    </div>
    <div class="item mx-4">
      <a href="#">New</a>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent, PropType} from 'vue';
import {ShaderCode} from '../shaders';

export default defineComponent({
  name: 'SdxHeader',

  data() {
    return {
      selected: null
    }
  },

  props: {
    shaderList: {
      required: true,
      type: Object as PropType<Array<ShaderCode>>
    }
  },

  methods: {
    selectShader(shader: ShaderCode) {
      this.selected = shader;

      this.$emit('compile', shader.code);
    }
  }
})
</script>

<style lang="scss" scoped>
.header {
  height: 70px;
  background-color: #EEEEEE;

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