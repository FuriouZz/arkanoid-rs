import { WASM } from "./wasm.js"

async function main() {
  // Load WASM
  const w = await WASM.load('./target/wasm32-unknown-unknown/debug/arkanoid-rust.wasm')
}

window.addEventListener('DOMContentLoaded', main)