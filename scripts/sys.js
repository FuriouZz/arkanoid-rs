import { decode_utf8 } from "./utils.js"
import { WASM } from "./wasm.js"

/**
 * @this WASM
 */
export function console_log(ptr, len) {
  const bytes = new Uint8Array(this.memory.buffer, ptr, len)
  console.log(decode_utf8(bytes))
}

/**
 * @this WASM
 */
export function console_warn(ptr, len) {
  const bytes = new Uint8Array(this.memory.buffer, ptr, len)
  console.warn(decode_utf8(bytes))
}