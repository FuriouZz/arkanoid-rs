const decoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true })
export function decode_utf8(bytes: Uint8Array) {
  return decoder.decode(bytes)
}

export async function load_wasm(path: string, importObject: Record<string, Record<string, WebAssembly.ImportValue>>) {
  const r0 = await fetch(path)
  const bytes = await r0.arrayBuffer()
  return WebAssembly.instantiate(bytes, importObject)
}

export function bind(obj: any, thisArg: any) {
  Object.entries(obj).forEach(([key, value]) => {
    if (typeof value === "function") obj[key] = value.bind(thisArg)
  })
  return obj
}