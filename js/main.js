function decode_utf8(bytes) {
  var i = 0, s = '';
  while (i < bytes.length) {
    var c = bytes[i++];
    if (c > 127) {
      if (c > 191 && c < 224) {
        if (i >= bytes.length)
          throw new Error('UTF-8 decode: incomplete 2-byte sequence');
        c = (c & 31) << 6 | bytes[i++] & 63;
      } else if (c > 223 && c < 240) {
        if (i + 1 >= bytes.length)
          throw new Error('UTF-8 decode: incomplete 3-byte sequence');
        c = (c & 15) << 12 | (bytes[i++] & 63) << 6 | bytes[i++] & 63;
      } else if (c > 239 && c < 248) {
        if (i + 2 >= bytes.length)
          throw new Error('UTF-8 decode: incomplete 4-byte sequence');
        c = (c & 7) << 18 | (bytes[i++] & 63) << 12 | (bytes[i++] & 63) << 6 | bytes[i++] & 63;
      } else throw new Error('UTF-8 decode: unknown multibyte start 0x' + c.toString(16) + ' at index ' + (i - 1));
    }
    if (c <= 0xffff) s += String.fromCharCode(c);
    else if (c <= 0x10ffff) {
      c -= 0x10000;
      s += String.fromCharCode(c >> 10 | 0xd800)
      s += String.fromCharCode(c & 0x3FF | 0xdc00)
    } else throw new Error('UTF-8 decode: code point 0x' + c.toString(16) + ' exceeds UTF-16 reach');
  }
  return s;
}


async function load_wasm(path, interface) {
  const r0 = await fetch(path)
  const bytes = await r0.arrayBuffer()
  return WebAssembly.instantiate(bytes, interface)
}

async function main() {
  const wasm = await WasmInterface.load('./target/wasm32-unknown-unknown/debug/arkanoid-rust.wasm')
  wasm.exports.main()
}

window.addEventListener('DOMContentLoaded', main)

class WasmInterface {

  wasm

  /**
   * @type {WebAssembly.Memory}
   */
  get memory() {
    return this.wasm.instance.exports.memory
  }

  /**
   * @type {Record<string, WebAssembly.ExportValue>}
   */
  get exports() {
    return this.wasm.instance.exports
  }

  createInterface() {
    return {
      env: {
        console_log: this.console_log.bind(this),
      }
    }
  }

  console_log(ptr, len) {
    const bytes = new Uint8Array(this.memory.buffer, ptr, len)
    console.log(decode_utf8(bytes))
  }

  /**
   * @param {string} path
   */
  static async load(path) {
    const wasm = new WasmInterface()
    const wasm_module = await load_wasm(path, wasm.createInterface())
    wasm.wasm = wasm_module
    return wasm
  }

}