import { ExportInterface } from "./export-interface"

async function main() {
  const x = await ExportInterface.load('./target/wasm32-unknown-unknown/debug/arkanoid-rust.wasm')
}

window.addEventListener('DOMContentLoaded', main)