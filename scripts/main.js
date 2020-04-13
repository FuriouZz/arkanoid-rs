import { ExportInterface } from "./export-interface.js"

async function main() {
  const x = await ExportInterface.load('./target/wasm32-unknown-unknown/debug/arkanoid-rust.wasm')
  x.instance.exports.main()
}

window.addEventListener('DOMContentLoaded', main)