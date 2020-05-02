import { walk,  } from "https://deno.land/std/fs/mod.ts";
import { globToRegExp, normalize } from "https://deno.land/std/path/mod.ts";

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

type LSOptions = {
  type: "file" | "directory"
  maxDepth: number
  includeRoot: boolean
}

async function *ls(pattern: string, path = ".", options: Partial<LSOptions> = {}) {
  const opts: LSOptions = Object.assign({
    type: "file",
    includeRoot: false,
    maxDepth: Infinity
  }, options)

  path = normalize(path)

  const w = walk(path, {
    match: [globToRegExp(pattern)],
    includeFiles: opts.type === "file",
    includeDirs: opts.type === "directory",
    maxDepth: opts.maxDepth,
  })

  for await (const entry of w) {
    const relative = entry.path.slice(path.length)
    if (opts.type === "file" && entry.isFile) {
      yield {
        relative,
        ...entry
      }
    } else if (opts.type === "directory" && entry.isDirectory) {
      if (!opts.includeRoot && relative.length === 0) continue
      yield {
        relative,
        ...entry
      }
    }
  }
}

async function read_files(pattern: string, path = ".") {
  const w = ls(pattern, path, {
    type: "file"
  })

  const files: Record<string, string> = {}
  for await (const entry of w) {
    files[`/${entry.relative}`] = await read_text(entry.path)
  }

  return files
}

const dirs = ls("**/*", "./public/scripts/", { type: "directory", maxDepth: 1, includeRoot: false })

for await (const dir of dirs) {
  const files = await read_files("**/*.ts", `./public/scripts/${dir.name}/`)
  const count = Object.keys(files).length
  if (count === 0) continue

  const [errors, emitted] = await Deno
  .bundle("/mod.ts", files, { lib: [] })

  if (errors) {
    for (const error of errors) {
      console.log(error.message)
      // console.log(`Err: ${error.scriptResourceName} [${error.lineNumber}:${error.startColumn}]`)
      // console.log(`${error.message}`)
      // console.log(`${error.sourceLine}\n`)
    }
  } else {
    write_text(`public/${dir.name}.js`, emitted)
    console.log(`public/scripts/${dir.name}/mod.ts compiled`)
  }
}