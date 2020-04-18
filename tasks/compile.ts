import { walk } from "https://deno.land/std/fs/mod.ts";
import { globToRegExp, normalize } from "https://deno.land/std/path/mod.ts";
import { assert } from "https://deno.land/std/testing/asserts.ts";

async function read_text(path: string) {
  const decoder = new TextDecoder("utf-8")
  const data = await Deno.readFile(path)
  return decoder.decode(data)
}

async function write_text(path: string, content: string) {
  const encoder = new TextEncoder()
  const data = encoder.encode(content)
  await Deno.writeFile(path, data)
}

async function read_files(pattern: string, path = ".") {
  path = normalize(path)

  const w = walk(path, {
    match: [globToRegExp(pattern)]
  })

  const files: Record<string, string> = {}
  for await (const entry of w) {
    const file = entry.filename.slice(path.length)
    files[`/${file}`] = await read_text(entry.filename)
  }
  return files
}

const files = await read_files("**/*.ts", "./public/scripts/")
const [errors, emitted] = await Deno
.bundle("/mod.ts", files, {
  lib: [ "dom", "esnext" ]
})

if (errors) {
  console.log(errors);
  for (const error of errors) {
    console.log(`Err: ${error.scriptResourceName} [${error.lineNumber}:${error.startColumn}]`)
    console.log(`${error.message}`)
    console.log(`${error.sourceLine}\n`)
  }
} else {
  write_text("public/wasm.js", emitted)
  console.log(`Compiled to "public/wasm.js"`)
}