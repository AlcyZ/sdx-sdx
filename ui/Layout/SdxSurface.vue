<!--suppress TypeScriptValidateTypes -->
<template>
  <div class="h-100 d-flex justify-content-center">
    <div class="surface-container d-flex flex-column">
      <canvas id="surface"></canvas>
      <div class="toolbar">
        <button @click="play" v-if="paused"><i class="bi bi-play"></i></button>
        <button @click="pause" v-else><i class="bi bi-pause"></i></button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';
import init, {Surface} from '../wasm/sdx_sdx';

export default defineComponent({
  name: 'SdxSurface',

  data() {
    return {
      time: 0,
      paused: false,
      surface: undefined,
      animationId: undefined,
    }
  },

  props: {
    code: {
      required: true,
      type: String,
    }
  },

  watch: {
    code(newValue: string): void {
      window.cancelAnimationFrame(this.animationId);
      this.surface.clear();
      this.start();
    }
  },

  methods: {
    play() {
      this.paused = false;

      this.animationId = window.requestAnimationFrame(this.render);
    },
    pause() {
      this.paused = true;
    },

    render(): void {
      if (this.paused) {
        return;
      }

      this.time += 1 / 60;
      this.surface.render(this.time);
      this.animationId = window.requestAnimationFrame(this.render);
    },

    start(): void {
      if (this.surface) {
        this.surface.free();
      }
      this.surface = Surface.new('#surface');
      this.surface.setup_fs_program(this.code);

      this.animationId = window.requestAnimationFrame(this.render);
    }
  },

  async mounted(): Promise<void> {
    await init();

    this.start();
  }
})
</script>

<style lang="scss" scoped>
@import "variables";

$toolbar-color: #444444;

.toolbar {
  background-color: $toolbar-color;

  button {
    color: #DDDDDD;
    background-color: lighten($toolbar-color, 10);

    border: 1px solid $toolbar-color;
    border-radius: 0;
    padding: 0.4rem 1rem;

    &:active {
      background-color: lighten($toolbar-color, 20);
    }
  }
}

#surface {
  max-height: $surface-height;

  height: $surface-height;
}
</style>