import { load_wasm, bind } from "./utils.js"
import * as Functions from "./fns.js"

export class WASM {

  renderer
  wasm
  get memory() { return this.wasm.instance.exports.memory }
  get exports() { return this.wasm.instance.exports }

  /**
   * @param {string} path
   */
  static async load(path) {
    const i = new WASM()
    const env = bind({...Functions}, i)
    const wasm = await load_wasm(path, { env })
    i.wasm = wasm
    return i
  }

}