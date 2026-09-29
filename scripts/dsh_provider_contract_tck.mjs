import assert from 'node:assert/strict'
import { existsSync } from 'node:fs'
import { mkdir, mkdtemp, readFile, rm } from 'node:fs/promises'
import { dirname, join, resolve } from 'node:path'
import { pathToFileURL } from 'node:url'

const repoRoot = resolve(dirname(new URL(import.meta.url).pathname), '..')
const closure = resolve(process.env.OPENMUSE_DSH_CLOSURE || join(repoRoot, 'target/dsh-closure'))
const modules = join(closure, 'node_modules')
const contract = JSON.parse(await readFile(join(repoRoot, 'contracts/dsh/0.1.7-rc.1.contract.json'), 'utf8'))

function packageRoot(name) {
  return join(modules, ...name.split('/'))
}

async function packageJson(name) {
  return JSON.parse(await readFile(join(packageRoot(name), 'package.json'), 'utf8'))
}

async function importPackage(name) {
  return import(pathToFileURL(join(packageRoot(name), 'lib/index.js')).href)
}

async function expectCode(promise, code) {
  await assert.rejects(promise, error => error?.code === code)
}

async function verifyClosure() {
  assert.equal((await packageJson('@deepseek-ai/dsh')).version, contract.dshVersion)
  for (const name of contract.requiredPackages) {
    assert.equal(existsSync(packageRoot(name)), true, `missing required package ${name}`)
    if (name.startsWith('@deepseek-ai/dsh-')) {
      assert.equal((await packageJson(name)).version, contract.dshVersion, `${name} version drift`)
    }
  }
  for (const name of contract.forbiddenProductionPackages) {
    assert.equal(existsSync(packageRoot(name)), false, `${name} must not enter the product closure`)
  }

  const basePatch = await readFile(join(packageRoot('@deepseek-ai/dsh-base'), 'cordis.patch.yml'), 'utf8')
  for (const [id, name] of Object.entries(contract.profileRows)) {
    assert.match(basePatch, new RegExp(`id: ${id}\\n\\s+name: ['\"]?${name.replaceAll('/', '\\/')}`))
  }
  const overlay = await readFile(join(repoRoot, 'contracts/dsh/providers/local/cordis.patch.yml'), 'utf8')
  for (const id of Object.keys(contract.profileRows)) {
    assert.match(overlay, new RegExp(`id: ${id}\\n`), `provider overlay does not replace ${id}`)
  }
  assert.equal(basePatch.includes('@deepseek-ai/dsh-agent-loop'), true)
  assert.equal(overlay.includes('agent-loop'), false, 'provider replacement must not modify Agent Loop')

  const sandboxFsSource = await readFile(join(packageRoot('@deepseek-ai/dsh-fs-sandbox'), 'lib/index.js'), 'utf8')
  assert.match(sandboxFsSource, /SandboxedFileSystem\s*=\s*class extends LocalFileSystem/)
  assert.equal(contract.capabilities.lspAdapterInProductClosure, existsSync(packageRoot('@deepseek-ai/dsh-lsp-stdio')))
}

async function runExecutionWorldTck() {
  const [{ Context }, { default: LocalFileSystem }, { default: LocalSubprocessRuntime }, { default: LocalBashExecutor }] = await Promise.all([
    importPackage('@deepseek-ai/cordis'),
    importPackage('@deepseek-ai/dsh-fs-local'),
    importPackage('@deepseek-ai/dsh-subprocess-local'),
    importPackage('@deepseek-ai/dsh-bash-local'),
  ])
  const base = await mkdtemp(join(repoRoot, '.dsh-provider-tck-'))
  const workspace = join(base, 'workspace')
  await mkdir(workspace)
  const ctx = new Context()
  const fibers = []
  try {
    fibers.push(await ctx.plugin(LocalFileSystem, { cwd: workspace }))
    fibers.push(await ctx.plugin(LocalSubprocessRuntime))
    fibers.push(await ctx.plugin(LocalBashExecutor, { cwd: workspace, graceMs: 100 }))

    const fromFs = await ctx.fs.resolve('from-fs.txt')
    await ctx.fs.writeText(fromFs, 'filesystem-to-shell')
    const catExecution = await ctx.shell.execute(ctx.shell.resolve({ command: 'cat from-fs.txt' }))
    const catResult = await catExecution.result()
    assert.equal(catResult.exitCode, 0)
    assert.equal(catResult.stdout.text, 'filesystem-to-shell')

    const writeExecution = await ctx.shell.execute(ctx.shell.resolve({ command: "printf 'shell-to-filesystem' > from-shell.txt" }))
    assert.equal((await writeExecution.result()).exitCode, 0)
    assert.equal(await ctx.fs.readText(await ctx.fs.resolve('from-shell.txt')), 'shell-to-filesystem')
    assert.equal(ctx.fs.processPath(await ctx.fs.resolve('from-shell.txt')), join(workspace, 'from-shell.txt'))

    const controller = new AbortController()
    const cancelled = await ctx.shell.execute(ctx.shell.resolve({ command: 'sleep 60', signal: controller.signal }))
    controller.abort('tck-cancel')
    const cancelledResult = await cancelled.result()
    assert.equal(cancelledResult.aborted, true)
    assert.equal(cancelledResult.timedOut, false)

    const background = await ctx.shell.execute(ctx.shell.resolve({ command: 'sleep 60', onExpiry: 'none' }))
    assert.equal(background.status, 'running')
    assert.equal(background.kill(), true)
    await background.done
    assert.equal(background.status, 'killed')

    await testTerminal(ctx, workspace)
    await testLspFraming(ctx, workspace)
  } finally {
    for (const fiber of fibers.reverse()) await fiber.dispose()
    await rm(base, { recursive: true, force: true })
  }
}

async function testTerminal(ctx, workspace) {
  const bash = await ctx.subprocess.resolveExecutable('bash')
  const terminal = await ctx.subprocess.spawnTerminal({
    argv: [bash, '--noprofile', '--norc'],
    cwd: workspace,
    rows: 24,
    cols: 80,
    terminalType: 'xterm-256color',
    graceMs: 100,
  })
  let output = ''
  terminal.output.on('data', chunk => { output += chunk.toString() })
  await terminal.write("printf 'OPENMUSE_PTY_OK\\n'; exit\n")
  const outcome = await terminal.done
  assert.equal(outcome.exitCode, 0)
  assert.equal(output.includes('OPENMUSE_PTY_OK'), true)
  await terminal.terminate()
}

async function testLspFraming(ctx, workspace) {
  const server = [
    "let b=Buffer.alloc(0);",
    "process.stdin.on('data',c=>{b=Buffer.concat([b,c]);const p=b.indexOf('\\r\\n\\r\\n');if(p<0)return;const m=/Content-Length: (\\d+)/i.exec(b.toString('ascii',0,p));if(!m)return;const n=Number(m[1]);if(b.length<p+4+n)return;const q=JSON.parse(b.toString('utf8',p+4,p+4+n));const body=Buffer.from(JSON.stringify({jsonrpc:'2.0',id:q.id,result:{capabilities:{hoverProvider:true}}}));process.stdout.write('Content-Length: '+body.length+'\\r\\n\\r\\n');process.stdout.write(body);});",
  ].join('')
  const handle = ctx.subprocess.spawn({
    argv: [process.execPath, '-e', server],
    cwd: workspace,
    stdio: { stdin: 'pipe', stdout: 'pipe', stderr: { maxBytes: 4096 } },
    graceMs: 100,
  })
  const request = Buffer.from(JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'initialize', params: {} }))
  const response = new Promise((resolveResponse, rejectResponse) => {
    let bytes = Buffer.alloc(0)
    handle.stdout.on('data', chunk => {
      bytes = Buffer.concat([bytes, chunk])
      const split = bytes.indexOf('\r\n\r\n')
      if (split < 0) return
      const match = /Content-Length: (\d+)/i.exec(bytes.toString('ascii', 0, split))
      if (!match) return
      const length = Number(match[1])
      if (bytes.length < split + 4 + length) return
      resolveResponse(JSON.parse(bytes.toString('utf8', split + 4, split + 4 + length)))
    })
    handle.done.catch(rejectResponse)
  })
  handle.stdin.end(Buffer.concat([
    Buffer.from(`Content-Length: ${request.length}\r\n\r\n`),
    request,
  ]))
  const value = await response
  assert.deepEqual(value.result, { capabilities: { hoverProvider: true } })
  handle.terminate()
  await handle.done
}

async function runSandboxTck() {
  const [{ Context }, { default: SessionProjectionRegistry }, { default: SandboxPolicyService }, { default: SandboxedFileSystem }] = await Promise.all([
    importPackage('@deepseek-ai/cordis'),
    importPackage('@deepseek-ai/dsh-session-projection'),
    importPackage('@deepseek-ai/dsh-sandbox-policy'),
    importPackage('@deepseek-ai/dsh-fs-sandbox'),
  ])
  const base = await mkdtemp(join(repoRoot, '.dsh-sandbox-tck-'))
  const workspace = join(base, 'workspace')
  const outside = join(base, 'outside')
  await mkdir(workspace)
  await mkdir(outside)
  const ctx = new Context()
  const fibers = []
  try {
    fibers.push(await ctx.plugin(SessionProjectionRegistry))
    fibers.push(await ctx.plugin(SandboxPolicyService, { mode: 'read-only', workspaceRoot: workspace }))
    fibers.push(await ctx.plugin(SandboxedFileSystem, { cwd: workspace }))
    const denied = await ctx.fs.resolve(join(workspace, 'denied.txt'))
    await expectCode(ctx.fs.writeText(denied, 'denied'), 'FS_SANDBOX_DENIED')
    assert.equal(existsSync(join(workspace, 'denied.txt')), false)

    const allowed = await ctx.fs.resolve(join(workspace, 'allowed.txt'))
    await ctx.fs.writeText(allowed, 'allowed', undefined, undefined, { mode: 'workspace-write', workspaceRoot: workspace })
    assert.equal(await ctx.fs.readText(allowed), 'allowed')

    const escape = await ctx.fs.resolve(join(outside, 'escape.txt'))
    await expectCode(
      ctx.fs.writeText(escape, 'escape', undefined, undefined, { mode: 'workspace-write', workspaceRoot: workspace }),
      'FS_SANDBOX_DENIED',
    )
    assert.equal(existsSync(join(outside, 'escape.txt')), false)
  } finally {
    for (const fiber of fibers.reverse()) await fiber.dispose()
    await rm(base, { recursive: true, force: true })
  }
}

async function runJobsTck() {
  const [{ Context }, { default: LocalJobRegistry }] = await Promise.all([
    importPackage('@deepseek-ai/cordis'),
    importPackage('@deepseek-ai/dsh-jobs-local'),
  ])
  const ctx = new Context()
  const fiber = await ctx.plugin(LocalJobRegistry, { pumpPollMs: 5 })
  const detach = ctx.jobs.attachController('openmuse-x0-tck')
  try {
    let settle
    const done = new Promise(resolveDone => { settle = resolveDone })
    const id = ctx.jobs.start({
      kind: 'bash',
      label: 'x0-contract-job',
      run(job) {
        job.append('job-output')
        return {
          cancel(reason) { settle({ status: 'killed', detail: reason }) },
          done,
        }
      },
    })
    assert.equal(ctx.jobs.readAt(id, 0).chunks.map(chunk => chunk.text).join(''), 'job-output')
    assert.equal(ctx.jobs.kill(id, undefined, 'tck-stop'), 'requested')
    const view = await ctx.jobs.wait(id, 1_000)
    assert.equal(view.status, 'killed')
    assert.match(view.detail, /tck-stop/)
  } finally {
    detach()
    await fiber.dispose()
  }
}

await verifyClosure()
await runExecutionWorldTck()
await runSandboxTck()
await runJobsTck()
console.log(JSON.stringify({
  schema: contract.schema,
  dshVersion: contract.dshVersion,
  status: 'passed',
  capabilities: contract.capabilities,
}))
