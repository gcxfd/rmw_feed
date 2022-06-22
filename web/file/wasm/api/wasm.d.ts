/* tslint:disable */
/* eslint-disable */
/**
*/
export function prepare(): void;
/**
*/
export class W {
  free(): void;
/**
* @returns {Promise<any>}
*/
  stop(): Promise<any>;
/**
* @param {string} url
* @param {Function} onopen
* @param {Function} onclose
* @returns {W}
*/
  static new(url: string, onopen: Function, onclose: Function): W;
/**
*/
  connect(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_w_free: (a: number) => void;
  readonly w_stop: (a: number) => number;
  readonly w_new: (a: number, b: number, c: number, d: number) => number;
  readonly w_connect: (a: number) => void;
  readonly prepare: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly closure2_externref_shim: (a: number, b: number, c: number) => void;
  readonly closure72_externref_shim: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly closure90_externref_shim: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
