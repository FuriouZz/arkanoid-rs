/// <reference lib="dom" />
/// <reference lib="esnext" />

let id = 0

export class Canvas {

  $canvas: HTMLCanvasElement
  private _id: number = 0

  constructor() {
    this._id = ++id

    this.$canvas = document.createElement('canvas')
    this.$canvas.setAttribute('tab-index', '0')
    this.$canvas.setAttribute('data-raw-handle', this._id.toString())
    document.body.appendChild(this.$canvas)
  }

  get id() {
    return this._id
  }

  get width() {
    return this.$canvas.width
  }

  get height() {
    return this.$canvas.height
  }

  ready() {
    window.dispatchEvent(new CustomEvent("canvas:ready", {
      detail: this.$canvas
    }))
  }

  resize(width: number, height: number) {
    this.$canvas.width = width * window.devicePixelRatio
    this.$canvas.height = height * window.devicePixelRatio
    this.$canvas.style.width = `${width}px`
    this.$canvas.style.height = `${height}px`
  }

}