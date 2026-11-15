// Fault-injection server for protocol/lifecycle tests, never used in a product.
import { writeFileSync } from 'node:fs';
import { spawn } from 'node:child_process';
import { createMessageConnection, StreamMessageReader, StreamMessageWriter } from 'vscode-jsonrpc/node.js';
const [mode, pidFile] = process.argv.slice(2);
if (pidFile) writeFileSync(pidFile, String(process.pid));
if (mode === 'worker') {
  const worker = spawn(process.execPath, ['-e', 'setInterval(() => {}, 1000)'], { stdio: 'ignore' });
  writeFileSync(pidFile + '.worker', String(worker.pid));
}
const connection = createMessageConnection(new StreamMessageReader(process.stdin), new StreamMessageWriter(process.stdout));
connection.onRequest('initialize', () => ({ capabilities: {
  positionEncoding: mode === 'utf8' ? 'utf-8' : 'utf-16',
  textDocumentSync: { openClose: true, change: 1 }, definitionProvider: true,
} }));
connection.onRequest('textDocument/definition', params => mode === 'hang' ? new Promise(() => {}) : [{
  targetUri: params.textDocument.uri,
  targetRange: { start: { line: 0, character: 0 }, end: { line: 2, character: 0 } },
  targetSelectionRange: { start: { line: 1, character: 0 }, end: { line: 1, character: 4 } },
}]);
connection.onRequest('shutdown', () => mode === 'hang' ? new Promise(() => {}) : null);
connection.onNotification('exit', () => process.exit(0));
connection.listen();
