import Watcher from "https://deno.land/x/denon/watcher.ts";
import { globToRegExp } from "https://deno.land/std/path/glob.ts";

async function getConfig() {
  const decoder = new TextDecoder("utf-8")
  const data = await Deno.readFile("denow.json")
  return JSON.parse(decoder.decode(data))
}

async function run(cmd: string[]) {
  console.log(`> ${cmd.join(' ')}`)
  const ps = Deno.run({ cmd })
  return ps.status()
}

const config = await getConfig()

const w = new Watcher(config.watch, {
  interval: config.interval,
  exts: config.ext.split(',').map((ext: string) => `.${ext}`),
  match: [ "*" ],
  skip: [],
})

const executors = Object.entries(config.execute).map(([match, cmds]) => {
  return [
    globToRegExp(match, { extended: true, globstar: false }),
    cmds
  ] as [RegExp, string[][]]
})

for (const [regex, cmds] of executors) {
  for (const cmd of cmds) {
    await run(cmd)
  }
}

for await (const changes of w) {
  if (config.fullscreen) {
    console.clear();
  }

  for (const change of changes) {
    for (const [regex, cmds] of executors) {
      if (regex.test(change.path)) {
        for (const cmd of cmds) {
          await run(cmd)
        }
        break
      }
    }
  }
}