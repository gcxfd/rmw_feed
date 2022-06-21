/* tslint:disable */
/* eslint-disable */
/**
*/
export class Ws {
  free(): void;
/**
*/
  connect(): void;
/**
* @param {string} url
* @returns {Ws}
*/
  static new(url: string): Ws;
/**
* @param {Function} next
*/
  req(next: Function): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_ws_free: (a: number) => void;
  readonly ws_connect: (a: number) => void;
  readonly ws_new: (a: number, b: number) => number;
  readonly ws_req: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly closure2_externref_shim: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
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
