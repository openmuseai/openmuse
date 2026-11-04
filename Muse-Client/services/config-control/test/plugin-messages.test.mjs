import test from 'node:test';
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
test('registered plugin contracts publish signed optional resources and reject changed arguments',()=>{
 const fixture=JSON.parse(execFileSync(process.execPath,[new URL('./plugin-messages-fixture.mjs',import.meta.url).pathname],{encoding:'utf8'}));
 assert.equal(fixture.snapshot.resources[0].kind,'pluginMessages');
 assert.equal(fixture.pack.sourceRevision,3);
});
