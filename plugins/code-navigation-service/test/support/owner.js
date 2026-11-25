import { mkdtemp, writeFile, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { NavigationError } from '../../lib/service.js';

export async function fixture(t) {
  const root = await mkdtemp(join(tmpdir(), 'openmuse-navigation-'));
  t.after(() => rm(root, { recursive: true, force: true }));
  await writeFile(join(root, 'main.ts'), 'import { greet } from "./定义";\nconst result = greet("你好🙂");\n');
  await writeFile(join(root, '定义.ts'), 'export function greet(name: string): string { return name; }\n');
  await writeFile(join(root, 'tsconfig.json'), JSON.stringify({ compilerOptions: { strict: true }, include: ['*.ts'] }));
  const rootUri = pathToFileURL(root).href;
  let allowed = true;
  const owner = {
    revoke: () => allowed = false,
    async authorizeAndRead(input) {
      if (!allowed) throw new NavigationError('DENIED');
      return { partition: 'account:generation1', workspaceKey: 'workspace', profile: 'typescript',
        rootUri, uri: pathToFileURL(join(root, 'main.ts')).href, languageId: 'typescript', version: 'v1',
        text: await readFile(join(root, 'main.ts'), 'utf8') };
    },
    async assertCurrent() { if (!allowed) throw new NavigationError('DENIED'); },
    async resolveLocation(location) {
      const expected = pathToFileURL(join(root, '定义.ts')).href;
      const same = pathToFileURL(join(root, 'main.ts')).href;
      if (!allowed || ![expected, same].includes(location.uri)) throw new NavigationError('DENIED');
      return { reference: { resource: 'authorized-target' }, version: 'target-v1', label: location.uri === expected ? '定义.ts' : 'main.ts',
        range: location.range, selectionRange: location.selectionRange };
    },
  };
  return { root, rootUri, owner, input: {
    reference: { resource: 'opaque-main' }, version: 'v1', operation: 'definition',
    position: { line: 1, character: 16 }, requestId: 'test-query',
  } };
}
