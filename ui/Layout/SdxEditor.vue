<template>
  <div class="ide-container">
    <div class="editor"></div>
    <div class="toolbar d-flex">
      <button @click="compile">
        <i class="bi bi-play"></i>
      </button>

      <div class="dropdown ms-auto">
        <a class="btn btn-secondary dropdown-toggle" href="#" role="button" id="dropdownMenuLink"
           data-bs-toggle="dropdown" aria-expanded="false">
          Shaders
        </a>

        <ul class="dropdown-menu" aria-labelledby="dropdownMenuLink">
          <li v-for="(shader, i) in shaderList" :key="i">
            <a href="#" class="dropdown-item" v-text="shader.name" @click="compileCode(shader.code)"></a>
          </li>
        </ul>
      </div>

    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent, PropType} from 'vue';
import CodeMirror from 'codemirror';
import 'codemirror/lib/codemirror.css';
import 'codemirror/mode/clike/clike';
import 'codemirror/theme/darcula.css';
import 'codemirror/addon/edit/closebrackets';
import {ShaderCode} from "../shaders";

const BREAKPOINT_MD = 768;
const getPageWidth = (): number => {
  return Math.max(
      document.body.scrollWidth,
      document.documentElement.scrollWidth,
      document.body.offsetWidth,
      document.documentElement.offsetWidth,
      document.documentElement.clientWidth
  );
}

interface EditorData {
  editor?: CodeMirror.Editor
}

export default defineComponent({
  name: 'SdxEditor',

  data(): EditorData {
    return {
      editor: undefined
    }
  },

  props: {
    code: {
      required: true,
      type: String,
    },
    shaderList: {
      required: true,
      type: Object as PropType<ShaderCode>
    }
  },

  watch: {
    code() {
      this.createIde();
    }
  },

  methods: {
    compile(): void {
      if (this.editor) {
        this.$emit('compile', this.editor.getValue());
      }
    },

    compileCode(code: string): void {
      this.$emit('compile', code);
    },

    createIde(): void {
      const editor = document.querySelector('.editor');

      if (editor) {
        editor.innerHTML = '';

        this.editor = CodeMirror(editor as HTMLElement, {
          mode: 'x-shader/x-fragment',
          value: this.code,
          theme: 'darcula',
          lineNumbers: true,
          autoCloseBrackets: true,
        });
        this.resizeIde();
      }
    },

    resizeIde(): void {
      const surface = document.querySelector('#surface');
      const codeMirrorElement = document.querySelector('.editor .CodeMirror');
      if (surface && codeMirrorElement) {
        const rect = surface.getBoundingClientRect();

        if (getPageWidth() >= BREAKPOINT_MD) {
          (codeMirrorElement as HTMLElement).style.height = `${rect.height}px`;
        }
      }
    }
  },

  mounted() {
    this.createIde();

    window.addEventListener('resize', this.resizeIde);
  }
});
</script>

<style lang="scss" scoped>
@import "toolbar";

.ide-container {
  overflow-x: hidden;
}

</style>

<style lang="scss">
.editor {
  .CodeMirror {
    background-color: green;
  }
}
</style>
