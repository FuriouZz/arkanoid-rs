import { decode_utf8 } from "./utils.ts"
import { WASM } from "./wasm.ts"

export function console_log(this: WASM, ptr: number, len: number) {
  const bytes = new Uint8Array(this.memory.buffer, ptr, len)
  console.log(decode_utf8(bytes))
}

export function console_warn(this: WASM, ptr: number, len: number) {
  const bytes = new Uint8Array(this.memory.buffer, ptr, len)
  console.warn(decode_utf8(bytes))
}

export function performance_now() {
  return performance.now()
}