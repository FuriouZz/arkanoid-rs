/// <reference lib="dom" />
/// <reference lib="esnext" />

export interface IEventHandler {
  on_resize(width: number, height: number): void;
  on_frame(): void;
  on_key_up(key: number): void;
  on_key_down(key: number): void;
  on_key_press(key: number): void;
  on_pointer_up(x: number, y: number): void;
  on_pointer_down(x: number, y: number): void;
  on_pointer_move(x: number, y: number): void;
}

export class EventBridge {

  raf = new RAF()

  constructor(private _handler: IEventHandler) {
    this._onkey = this._onkey.bind(this)
    this._ready = this._ready.bind(this)
    this._resize = this._resize.bind(this)
    this._frame = this._frame.bind(this)
    this._onpointer = this._onpointer.bind(this)

    // @ts-ignore
    window.addEventListener('canvas:ready', this._ready, { once: true })

    this.raf.start()
  }

  _ready(e: CustomEvent) {
    this.enable(e.detail)
  }

  enable($canvas: HTMLCanvasElement) {
    $canvas.addEventListener('keyup', this._onkey)
    $canvas.addEventListener('keydown', this._onkey)
    $canvas.addEventListener('keypress', this._onkey)
    $canvas.addEventListener('pointerup', this._onpointer)
    $canvas.addEventListener('pointerdown', this._onpointer)
    $canvas.addEventListener('pointermove', this._onpointer)
    window.addEventListener("resize", this._resize)
    this._resize()
    this.raf.listeners.add(this._frame)
  }

  disable($canvas: HTMLCanvasElement) {
    $canvas.removeEventListener('keyup', this._onkey)
    $canvas.removeEventListener('keydown', this._onkey)
    $canvas.removeEventListener('keypress', this._onkey)
    $canvas.removeEventListener('pointerup', this._onpointer)
    $canvas.removeEventListener('pointerdown', this._onpointer)
    $canvas.removeEventListener('pointermove', this._onpointer)
    window.removeEventListener('resize', this._resize)
    this.raf.listeners.delete(this._frame)
  }

  private _resize() {
    this._handler.on_resize(Math.floor(window.innerWidth), Math.floor(window.innerHeight))
  }

  private _frame() {
    this._handler.on_frame()
  }

  private _onkey(e: KeyboardEvent) {
    switch (e.type) {
      case "canvas:keydown":
        {
          this._handler.on_key_down(e.keyCode)
          break
        }
      case "canvas:keyup":
        {
          this._handler.on_key_up(e.keyCode)
          break
        }
      case "canvas:keypress":
        {
          this._handler.on_key_press(e.keyCode)
          break
        }
    }
  }

  private _onpointer(e: PointerEvent) {
    switch (e.type) {
      case "canvas:pointerdown":
        {
          this._handler.on_pointer_down(Math.floor(e.clientX), Math.floor(e.clientY))
          break
        }
      case "canvas:pointerup":
        {
          this._handler.on_pointer_up(Math.floor(e.clientX), Math.floor(e.clientY))
          break
        }
      case "canvas:pointermove":
        {
          this._handler.on_pointer_move(Math.floor(e.clientX), Math.floor(e.clientY))
          break
        }
    }
  }

}

export class RAF {
  paused = true
  listeners = new Set<(dt: number, time: number) => void>()
  time = 0
  dt = 0

  start() {
    if (this.paused) {
      this.paused = false
      this.update()
    }
  }

  stop() {
    if (!this.paused) {
      this.paused = true
    }
  }

  update() {
    if (!this.paused) {
      window.requestAnimationFrame(this.update.bind(this))
      const time = performance.now()
      this.dt = time - this.time
      this.time += this.dt
      for (const listener of this.listeners) {
        listener(this.dt, this.time)
      }
    }
  }
}
