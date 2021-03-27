<!--suppress TypeScriptValidateTypes -->
<template>
  <div class="render-surface d-flex flex-column">
    <canvas id="surface"></canvas>
    <div class="toolbar d-flex">
      <button @click="play" v-if="paused"><i class="bi bi-play"></i></button>
      <button @click="pause" v-else><i class="bi bi-pause"></i></button>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';
import init, {Surface} from '../wasm/sdx_sdx';

interface Position2D {
  x: number;
  y: number;
}

const getCursorPosition = (canvas: HTMLCanvasElement, event: MouseEvent): Position2D => {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = rect.height - (event.clientY - rect.top);

  return {x, y};
}

const getTouchPosition = (canvas: HTMLCanvasElement, event: TouchEvent): Position2D => {
  const rect = canvas.getBoundingClientRect();
  const x = event.touches[0].clientX - rect.left;
  const y = rect.height - (event.touches[0].clientY - rect.top);

  return {x, y};
}

interface SurfaceData {
  time: number;
  paused: boolean;
  surface?: Surface,
  animationId?: number;
  canvas?: HTMLCanvasElement,
  mousePressed: boolean;
  touchPressed: boolean;
}

export default defineComponent({
  name: 'SdxSurface',

  data(): SurfaceData {
    return {
      time: 0,
      paused: false,
      surface: undefined,
      animationId: undefined,
      canvas: undefined,
      mousePressed: false,
      touchPressed: false,
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
      window.cancelAnimationFrame(this.animationId as number);
      this.surface?.clear();
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
      this.surface?.render(this.time);
      this.animationId = window.requestAnimationFrame(this.render);
    },

    start(): void {
      this.surface?.free();
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

    touchEnd(): void {
      this.touchPressed = false;
    },

    touchStart(event: TouchEvent): void {
      this.touchPressed = true;
      this.updateMousePosition(event);
    },

    touchMove(event: TouchEvent): void {
      if (!this.touchPressed) {
        return;
      }
      this.updateMousePosition(event);
    },

    updateMousePosition(event: MouseEvent | TouchEvent): void {
      if (this.surface) {
        const newPos = event instanceof MouseEvent
            ? getCursorPosition(this.canvas as HTMLCanvasElement, event)
            : getTouchPosition(this.canvas as HTMLCanvasElement, event);

        this.surface.update_mouse_pos(newPos.x, newPos.y);
      }
    }
  },

  async mounted(): Promise<void> {
    const canvas = document.getElementById('surface');
    if (!canvas || !(canvas instanceof HTMLCanvasElement)) {
      console.error('could not find canvas with id #surface');
      return;
    }

    canvas.addEventListener('mousedown', this.mouseDown);
    canvas.addEventListener('mouseup', this.mouseUp);
    canvas.addEventListener('mousemove', this.mouseMove);
    canvas.addEventListener('touchstart', this.touchStart);
    canvas.addEventListener('touchend', this.touchEnd);
    canvas.addEventListener('touchmove', this.touchMove);

    this.canvas = canvas;
    await init();
    this.start();
  }
})
</script>

<style lang="scss" scoped>
@import "toolbar";
@import "../../node_modules/bootstrap/scss/functions";
@import "../../node_modules/bootstrap/scss/variables";
@import "../../node_modules/bootstrap/scss/mixins";

@include media-breakpoint-up(xl) {
  .render-surface {
    min-height: 600px;
  }
}

#surface {
  width: 100%;
  height: 100%;
  max-height: 375px;
  cursor: pointer;

  @include media-breakpoint-up(xl) {
    max-height: 1000px;
  }
}
</style>
