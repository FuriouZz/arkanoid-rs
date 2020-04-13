import { WASM } from "./wasm.js"
import { Renderer } from "./renderer.js"

async function main() {

  // Load WASM
  const w = await WASM.load('./target/wasm32-unknown-unknown/debug/arkanoid-rust.wasm')

  // Create canvas context
  w.renderer = new Renderer()

  w.exports.main()
}

window.addEventListener('DOMContentLoaded', main)