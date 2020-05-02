async function run(cmd: string[], cwd?: string) {
  console.log(`> ${cmd.join(' ')}`)
  const ps = Deno.run({ cmd, cwd })
  await ps.status()
}

const command = Deno.args[0]

if (command == "compile") {
  await run([ "deno", "--allow-read", "--allow-write", "--unstable", "tasks/compile.ts" ])
} else if (command == "server") {
  await run([ "deno", "--allow-net", "--allow-read", "../tasks/server.ts" ], 'public')
} else {
  await run([ "deno", "--allow-read", "--allow-run", "tasks/watch.ts" ])
}