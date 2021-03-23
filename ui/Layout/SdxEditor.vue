<template>
  <div class="d-flex flex-column h-100">
    <div class="editor"></div>
    <div class="toolbar">
      <button @click="compile">Compile</button>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';
import CodeMirror from 'codemirror';
import 'codemirror/lib/codemirror.css';
import 'codemirror/mode/clike/clike';
import 'codemirror/theme/darcula.css';
import 'codemirror/addon/edit/closebrackets';

export default defineComponent({
  name: 'SdxEditor',

  data() {
    return {
      editor: null
    }
  },

  props: {
    startValue: {
      required: true,
      type: String,
    }
  },

  methods: {
    compile() {
      if (this.editor) {
        this.$emit('compile', this.editor.getValue());
      }
    }
  },

  mounted() {
    const editor = document.querySelector('.editor');

    if (editor) {
      this.editor = CodeMirror(editor, {
        mode: 'x-shader/x-fragment',
        value: this.startValue,
        theme: 'darcula',
        lineNumbers: true,
        autoCloseBrackets: true,
      });
    }
  }
});
</script>

<style lang="scss" scoped>
$toolbar-color: #444444;

.toolbar {
  background-color: $toolbar-color;

  button {
    color: #dddddd;
    background-color: lighten($toolbar-color, 10);

    border: 1px solid $toolbar-color;
    border-radius: 0;
    //outline: none;
    padding: 0.4rem 1rem;

    &:active {
      background-color: lighten($toolbar-color, 20);
    }
  }
}

</style>

<style lang="scss">
@import "variables";

.editor {
  .CodeMirror {
    height: $surface-height;
  }
}
</style>