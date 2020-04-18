import { decode_utf8 } from "./utils.js"
import { WASM } from "./wasm.js"

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

/**
 * @this WASM
 */
export function canvas_restore() {
  this.ctx.restore()
}

/**
 * @this WASM
 */
export function canvas_save() {
  this.ctx.save()
}

/**
 * @this WASM
 */
export function canvas_set_transform(a, b, c, d, e, f) {
  this.ctx.setTransform(a, b, c, d, e, f)
}

/**
 * @this WASM
 */
export function canvas_reset_transform() {
  this.ctx.setTransform(1, 0, 0, 1, 0, 0)
  this.ctx.scrollPathIntoView
}

/**
 * @this WASM
 */
export function canvas_stroke() {
  this.ctx.stroke()
}

/**
 * @this WASM
 */
export function canvas_fill() {
  this.ctx.fill()
}

/**
 * @this WASM
 */
export function canvas_fill_outside() {
  this.ctx.fill("evenodd")
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 * @param {number} radiusX
 * @param {number} radiusY
 * @param {number} rotation
 * @param {number} startAngle
 * @param {number} endAngle
 * @param {boolean} anticlockwise
 */
export function canvas_ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise) {
  this.ctx.ellipse(x, y, radiusX, radiusY, rotation, startAngle, endAngle, anticlockwise)
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 * @param {number} radius
 */
export function canvas_circle(x, y, radius) {
  this.ctx.arc(x, y, radius, 0, Math.PI * 2)
}

/**
 * @this WASM
 *
 * @param {number} alpha
 */
export function canvas_global_alpha(alpha) {
  this.ctx.globalAlpha = alpha
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 */
export function canvas_move_to(x, y) {
  this.ctx.moveTo(x, y)
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 */
export function canvas_line_to(x, y) {
  this.ctx.lineTo(x, y)
}

/**
 * @this WASM
 */
export function canvas_begin_path() {
  this.ctx.beginPath()
}

/**
 * @this WASM
 */
export function canvas_close_path() {
  this.ctx.closePath()
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 * @param {number} radius
 * @param {number} startAngle
 * @param {number} endAngle
 * @param {boolean} anticlockwise
 */
export function canvas_arc(x, y, radius, startAngle, endAngle, anticlockwise) {
  this.ctx.arc(x, y, radius, startAngle, endAngle, anticlockwise)
}

/**
 * @this WASM
 *
 * @param {number} cpx
 * @param {number} cpy
 * @param {number} x
 * @param {number} y
 */
export function canvas_quadratic_curve_to(cpx, cpy, x, y) {
  this.ctx.quadraticCurveTo(cpx, cpy, x, y)
}

/**
 * @this WASM
 *
 * @param {number} cp1x
 * @param {number} cp1y
 * @param {number} cp2x
 * @param {number} cp2y
 * @param {number} x
 * @param {number} y
 */
export function canvas_bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y) {
  this.ctx.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y)
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 * @returns {boolean}
 */
export function canvas_is_point_inside_path(x, y) {
  return this.ctx.isPointInPath(x, y, "nonzero")
}

/**
 * @this WASM
 *
 * @param {number} x
 * @param {number} y
 * @returns {boolean}
 */
export function canvas_is_point_outside_path(x, y) {
  return this.ctx.isPointInPath(x, y, "evenodd")
}

/**
 * @this WASM
 *
 * @returns {number}
 */
export function canvas_create_gradient() {
  const index = this.gradients.length
  this.gradients[index] = null
  return index
}

/**
 * @this WASM
 *
 * @param {number} index
 * @param {number} x0
 * @param {number} y0
 * @param {number} x1
 * @param {number} y1
 * @returns {number}
 */
export function canvas_linear_gradient(index, x0, y0, x1, y1) {
  const gradient = this.ctx.createLinearGradient(x0, y0, x1, y1)
  this.gradients[index] = gradient
}

/**
 * @this WASM
 * @param {number} x0
 * @param {number} y0
 * @param {number} r0
 * @param {number} x1
 * @param {number} y1
 * @param {number} r1
 * @returns {number}
 */
export function canvas_radial_gradient(index, x0, y0, r0, x1, y1, r1) {
  const gradient = this.ctx.createRadialGradient(x0, y0, r0, x1, y1, r1)
  this.gradients[index] = gradient
}

/**
 * @this WASM
 *
 * @param {number} index
 */
export function canvas_fill_style_gradient(index) {
  if (this.gradients[index]) {
    this.ctx.fillStyle = this.gradients[index]
  }
}

/**
 * @this WASM
 *
 * @param {number} index
 */
export function canvas_stroke_style_gradient(index) {
  if (this.gradients[index]) {
    this.ctx.strokeStyle = this.gradients[index]
  }
}

/**
 * @this WASM
 *
 * @param {number} index
 * @param {number} offset
 * @param {number} color_ptr
 * @param {number} color_len
 */
export function canvas_add_color_stop(index, offset, color_ptr, color_len) {
  if (this.gradients[index]) {
    const bytes = new Uint8Array(this.memory.buffer, color_ptr, color_len)
    const color = decode_utf8(bytes)
    const gradient = this.gradients[index]
    gradient.addColorStop(offset, color)
  }
}