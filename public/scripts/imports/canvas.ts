let id = 0

export class Canvas {

  canvas: HTMLCanvasElement
  private _id: number = 0

  constructor() {
    this._id = ++id

    this.canvas = document.createElement('canvas')
    this.canvas.setAttribute('tab-index', '0')
    this.canvas.setAttribute('data-raw-handle', this._id.toString())
    document.body.appendChild(this.canvas)
  }

  get id() {
    return this._id
  }

  get width() {
    return this.canvas.width
  }

  get height() {
    return this.canvas.height
  }

  resize(width: number, height: number) {
    this.canvas.width = width
    this.canvas.height = height
    this.canvas.style.width = `${this.canvas.width}px`
    this.canvas.style.height = `${this.canvas.height}px`
  }

}