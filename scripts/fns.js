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

/**
 * @this WASM
 */
export function canvas_clear() {
  this.ctx.clearRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height)
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 * @param {number} width
 * @param {number} height
 */
export function canvas_fill_rect(x, y, width, height) {
  this.ctx.fillRect(x, y, width, height)
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 * @param {number} width
 * @param {number} height
 */
export function canvas_fill_style(ptr, len) {
  const bytes = new Uint8Array(this.memory.buffer, ptr, len)
  this.ctx.fillStyle = decode_utf8(bytes)
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 * @param {number} width
 * @param {number} height
 */
export function canvas_stroke_rect(x, y, width, height) {
  this.ctx.strokeRect(x, y, width, height)
}

/**
 * @this WASM
 *
 * @param {number} ptr
 * @param {number} len
 */
export function canvas_stroke_style(ptr, len) {
  const bytes = new Uint8Array(this.memory.buffer, ptr, len)
  this.ctx.strokeStyle = decode_utf8(bytes)
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 */
export function canvas_translate(x, y) {
  this.ctx.translate(x, y)
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 */
export function canvas_scale(x, y) {
  this.ctx.scale(x, y)
}

/**
 * @this WASM
 *
 * @param {number} angle
 */
export function canvas_rotate(angle) {
  this.ctx.rotate(angle)
}