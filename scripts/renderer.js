export class Renderer {

  constructor() {
    this.onResize = this.onResize.bind(this)

    const $canvas = document.createElement('canvas')
    document.body.appendChild($canvas)
    $canvas.style.cssText = `position: fixed; top: 0; left: 0;`
    this.ctx = $canvas.getContext('2d')
  }

  enable() {
    window.addEventListener('resize', this.onResize)
    this.onResize()
  }

  disable() {
    window.removeEventListener('resize', this.onResize)
  }

  onResize() {
    $canvas.width = window.innerWidth
    $canvas.height = window.innerHeight
    $canvas.style.width = `${window.innerWidth}px`
    $canvas.style.height = `${window.innerHeight}px`
  }

}