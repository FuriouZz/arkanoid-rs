/// <reference lib="dom" />
/// <reference lib="esnext" />

export * from "./canvas.ts"

export function now() {
  return performance.now()
}

export function random() {
  return Math.random()
}