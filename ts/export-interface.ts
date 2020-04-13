import { decode_utf8, load_wasm } from "./utils"

export class ExportInterface {

  public wasm: WebAssembly.WebAssemblyInstantiatedSource

  get memory() {
    return this.instance.exports.memory as WebAssembly.Memory
  }

  get instance() {
    return this.wasm.instance as WebAssembly.WebAssemblyInstantiatedSource['instance']
  }

  console_log(ptr: number, len: number) {
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

  static async load(path: string) {
    const x = new ExportInterface()
    const wasm = await load_wasm(path, x.export())
    x.wasm = wasm
    return x
  }

}