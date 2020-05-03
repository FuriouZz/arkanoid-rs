import Watcher from "https://deno.land/x/denon/watcher.ts";
import { globToRegExp, parse } from "https://deno.land/std/path/mod.ts";
import { walkSync } from "https://deno.land/std/fs/mod.ts";

type ConfigExecutor = {
  env?: Record<string, string>
  args: string[]
}

type Config = {
  watch: string[]
  ext: string
  skip: string
  execute: Record<string, ConfigExecutor[]>
  fullscreen: boolean
  interval: number
}

type CommandList = Record<string, { args: string[], env?: Record<string, string> }>

async function getConfig() {
  const decoder = new TextDecoder("utf-8")
  const data = await Deno.readFile("denow.json")
  return JSON.parse(decoder.decode(data)) as Config
}

function t(template: string, data: Record<string, any>) {
  const keys = Object.keys(data)
  const values = Object.values(data)
  const f = new Function(...keys, `return \`${template}\``)
  return f.apply(null, values)
}

async function run(cmd: ConfigExecutor) {
  console.log(`> ${cmd.args.join(' ')}`)
  const ps = Deno.run({ cmd: cmd.args, env: cmd.env || {} })
  return ps.status()
}

function get_commands(path: string, c: CommandList) {
  for (const [regex, cmds] of executors) {
    if (regex.test(path)) {
      for (const cmd of cmds) {
        const parsed = parse(path)
        const args = cmd.args.map(a => t(a, parsed))
        const key = args.join(' ')

        c[key] = {
          args: args,
          env: cmd.env
        }
      }
    }
  }
}

async function run_commands(it: Iterable<{ path: string }>) {
  const list: CommandList = {}

  for (const entry of it) {
    get_commands(entry.path, list)
  }

  for (const cmd of Object.values(list)) {
    await run(cmd)
  }
}

const config = await getConfig()

const executors = Object.entries(config.execute).map(([match, cmds]) => {
  return [
    globToRegExp(match, { extended: true, globstar: false }),
    cmds
  ] as [RegExp, ConfigExecutor[]]
})

const ww = walkSync('.', {
  match: config.watch.map(w => globToRegExp(`${w}/**/*.{${config.ext}}`, { extended: true, globstar: false }))
})

await run_commands(ww)

const w = new Watcher(config.watch, {
  interval: config.interval,
  exts: config.ext.split(','),
  match: [ "*" ],
  skip: config.skip.split(','),
})

for await (const changes of w) {
  if (config.fullscreen) {
    console.clear();
  }

  await run_commands(changes)
}