import { load_wasm, decode_utf8 } from "./utils.js"

export class ExportInterface {

  /**
   * @type {WebAssembly.WebAssemblyInstantiatedSource}
   */
  wasm

  /**
   * @type {WebAssembly.Memory}
   */
  get memory() {
    return this.instance.exports.memory
  }

  /**
   * @type {WebAssembly.WebAssemblyInstantiatedSource.instance}
   */
  get instance() {
    return this.wasm.instance
  }

  /**
   * @param {number} ptr
   * @param {number} len
   */
  console_log(ptr, len) {
    const bytes = new Uint8Array(this.memory.buffer, ptr, len)
    console.log(decode_utf8(bytes))
  }

  export() {
    return {
      env: {
        console_log: this.console_log.bind(this)
      }
    }
  }

  /**
   * @param {string} path
   */
  static async load(path) {
    const x = new ExportInterface()
    const wasm = await load_wasm(path, x.export())
    x.wasm = wasm
    return x
  }

}