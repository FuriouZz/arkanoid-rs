import { decode_utf8 } from "./utils.js"

export function console_log(ptr, len) {
  const bytes = new Uint8Array(this.memory.buffer, ptr, len)
  console.log(decode_utf8(bytes))
}

export function canvas_clear() {
  this.renderer.ctx.clearRect(0, 0, this.renderer.ctx.canvas.width, this.renderer.ctx.canvas.height)
}

export function canvas_fill_rect(x, y, width, height) {
  this.renderer.ctx.fillRect(x, y, width, height)
}

export function canvas_fill_style(ptr, len) {
  const bytes = new Uint8Array(this.memory.buffer, ptr, len)
  this.renderer.ctx.fillStyle = decode_utf8(bytes)
}