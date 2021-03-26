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

const getCursorPosition = (canvas: HTMLCanvasElement, event: MouseEvent): { x: number, y: number } => {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = rect.height - (event.clientY - rect.top);

  return {x, y};
}

export default defineComponent({
  name: 'SdxSurface',

  data() {
    return {
      time: 0,
      paused: false,
      surface: undefined,
      animationId: undefined,
      canvas: null,
      mousePressed: false,
    }
  },

  props: {
    code: {
      required: true,
      type: String,
    }
  },

  watch: {
    code(): void {
      window.cancelAnimationFrame(this.animationId);
      if (this.canvas) {
        this.canvas.removeEventListener('mouse', this.updateMousePosition);
      }
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
      this.surface = Surface.new('#surface', this.code);
      this.animationId = window.requestAnimationFrame(this.render);
    },

    mouseUp(): void {
      this.mousePressed = false;
    },

    mouseDown(event: MouseEvent): void {
      this.mousePressed = true;
      this.updateMousePosition(event);
    },

    mouseMove(event: MouseEvent): void {
      if (!this.mousePressed) {
        return;
      }
      this.updateMousePosition(event);
    },

    updateMousePosition(event: MouseEvent): void {
      if (this.surface) {
        const newPos = getCursorPosition(this.canvas, event);
        this.surface.update_mouse_pos(newPos.x, newPos.y);
      }
    }
  },

  async mounted(): Promise<void> {
    const canvas = document.getElementById('surface');
    if (!canvas) {
      console.error('could not find canvas with id #surface');
      return;
    }
    canvas.addEventListener('mousedown', this.mouseDown);
    canvas.addEventListener('mouseup', this.mouseUp);
    canvas.addEventListener('mousemove', this.mouseMove);

    this.canvas = canvas;
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

  cursor: pointer;
}
</style>