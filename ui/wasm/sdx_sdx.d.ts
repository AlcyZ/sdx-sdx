/* tslint:disable */
/* eslint-disable */
/**
*/
export class Surface {
  free(): void;
/**
* @param {string} query_selector
* @returns {Surface}
*/
  static new(query_selector: string): Surface;
/**
* @param {string} fs_code
*/
  setup_fs_program(fs_code: string): void;
/**
* @param {number} time
*/
  render(time: number): void;
/**
*/
  clear(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_surface_free: (a: number) => void;
  readonly surface_new: (a: number, b: number) => number;
  readonly surface_setup_fs_program: (a: number, b: number, c: number) => void;
  readonly surface_render: (a: number, b: number) => void;
  readonly surface_clear: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
