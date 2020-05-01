let canvas: HTMLCanvasElement

export function canvas_create() {
  canvas = document.createElement('canvas')
  canvas.setAttribute('data-raw-handle', '1')
  document.body.appendChild(canvas)
}

export function canvas_get_size() {
  return [canvas.width, canvas.height]
}