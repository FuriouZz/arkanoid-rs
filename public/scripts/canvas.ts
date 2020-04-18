import { decode_utf8 } from "./utils.ts"
import { WASM } from "./wasm.ts"

export function canvas_clear(this: WASM) {
  this.ctx.clearRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height)
}

export function canvas_fill_rect(this: WASM, x: number, y: number, width: number, height: number) {
  this.ctx.fillRect(x, y, width, height)
}

export function canvas_fill_style(this: WASM, ptr: number, len: number) {
  const bytes = new Uint8Array(this.memory.buffer, ptr, len)
  this.ctx.fillStyle = decode_utf8(bytes)
}

export function canvas_stroke_rect(this: WASM, x: number, y: number, width: number, height: number) {
  this.ctx.strokeRect(x, y, width, height)
}

export function canvas_stroke_style(this: WASM, ptr: number, len: number) {
  const bytes = new Uint8Array(this.memory.buffer, ptr, len)
  this.ctx.strokeStyle = decode_utf8(bytes)
}

export function canvas_translate(this: WASM, x: number, y: number) {
  this.ctx.translate(x, y)
}

export function canvas_scale(this: WASM, x: number, y: number) {
  this.ctx.scale(x, y)
}

export function canvas_rotate(this: WASM, angle: number) {
  this.ctx.rotate(angle)
}

export function canvas_restore(this: WASM) {
  this.ctx.restore()
}

export function canvas_save(this: WASM) {
  this.ctx.save()
}

export function canvas_set_transform(this: WASM, a: number, b: number, c: number, d: number, e: number, f: number) {
  this.ctx.setTransform(a, b, c, d, e, f)
}

export function canvas_reset_transform(this: WASM) {
  this.ctx.setTransform(1, 0, 0, 1, 0, 0)
  this.ctx.scrollPathIntoView
}

export function canvas_stroke(this: WASM) {
  this.ctx.stroke()
}

export function canvas_fill(this: WASM) {
  this.ctx.fill()
}

export function canvas_fill_outside(this: WASM) {
  this.ctx.fill("evenodd")
}

export function canvas_ellipse(this: WASM, x: number, y: number, radiusX: number, radiusY: number, rotation: number, startAngle: number, endAngle: number, anticlockwise: boolean) {
  this.ctx.ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise)
}

export function canvas_circle(this: WASM, x: number, y: number, radius: number) {
  this.ctx.arc(x, y, radius, 0, Math.PI * 2)
}

export function canvas_global_alpha(this: WASM, alpha: number) {
  this.ctx.globalAlpha = alpha
}

export function canvas_move_to(this: WASM, x: number, y: number) {
  this.ctx.moveTo(x, y)
}

export function canvas_line_to(this: WASM, x: number, y: number) {
  this.ctx.lineTo(x, y)
}

export function canvas_begin_path(this: WASM) {
  this.ctx.beginPath()
}

export function canvas_close_path(this: WASM) {
  this.ctx.closePath()
}

export function canvas_arc(this: WASM, x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise: boolean) {
  this.ctx.arc(x, y, radius, startAngle, endAngle, anticlockwise)
}

export function canvas_quadratic_curve_to(this: WASM, cpx: number, cpy: number, x: number, y: number) {
  this.ctx.quadraticCurveTo(cpx, cpy, x, y)
}

export function canvas_bezier_curve_to(this: WASM, cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number) {
  this.ctx.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y)
}

export function canvas_is_point_inside_path(this: WASM, x: number, y: number) {
  return this.ctx.isPointInPath(x, y, "nonzero")
}

export function canvas_is_point_outside_path(this: WASM, x: number, y: number) {
  return this.ctx.isPointInPath(x, y, "evenodd")
}

export function canvas_create_gradient(this: WASM) {
  const index = this.gradients.length
  this.gradients[index] = null
  return index
}

export function canvas_linear_gradient(this: WASM, index: number, x0: number, y0: number, x1: number, y1: number) {
  const gradient = this.ctx.createLinearGradient(x0, y0, x1, y1)
  this.gradients[index] = gradient
}

export function canvas_radial_gradient(this: WASM, index: number, x0: number, y0: number, r0: number, x1: number, y1: number, r1: number) {
  const gradient = this.ctx.createRadialGradient(x0, y0, r0, x1, y1, r1)
  this.gradients[index] = gradient
}

export function canvas_fill_style_gradient(this: WASM, index: number) {
  if (this.gradients[index]) {
    this.ctx.fillStyle = this.gradients[index] as CanvasGradient
  }
}

export function canvas_stroke_style_gradient(this: WASM, index: number) {
  if (this.gradients[index]) {
    this.ctx.strokeStyle = this.gradients[index] as CanvasGradient
  }
}

export function canvas_add_color_stop(this: WASM, index: number, offset: number, color_ptr: number, color_len: number) {
  if (this.gradients[index]) {
    const bytes = new Uint8Array(this.memory.buffer, color_ptr, color_len)
    const color = decode_utf8(bytes)
    const gradient = this.gradients[index] as CanvasGradient
    gradient.addColorStop(offset, color)
  }
}