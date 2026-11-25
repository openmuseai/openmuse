// Same bounded MessageFormat subset as openmuse_i18n; data never becomes code.
export function parseMessage(source) {
  if (typeof source !== 'string' || source.length > 16000) throw Error('Message is too large');
  let at = 0, budget = 0;
  const parameters = new Set();
  const space = () => { while (/\s/.test(source[at] || '') && at < source.length) at++; };
  const consume = token => { space(); if (source[at] !== token) return false; at++; return true; };
  const expect = token => { if (!consume(token)) throw Error(`Expected ${token} at ${at}`); };
  const word = () => { space(); const start = at; while (at < source.length && !/[\s,{}]/.test(source[at])) at++; return source.slice(start, at); };
  function parse(depth = 0, plural = false) {
    if (depth > 8) throw Error('Message nesting limit exceeded');
    const nodes = []; let text = '';
    const flush = () => { if (text) nodes.push(['text', text]); text = ''; };
    while (at < source.length) {
      if (++budget > 20000) throw Error('Message complexity exceeded');
      const c = source[at];
      if (c === '}') break;
      if (c === "'") {
        if (source[at + 1] === "'") { text += "'"; at += 2; continue; }
        if ('{}#'.includes(source[at + 1] || '\0')) {
          at++; let closed = false;
          while (at < source.length) {
            if (source[at] === "'") {
              if (source[at + 1] === "'") { text += "'"; at += 2; continue; }
              at++; closed = true; break;
            }
            text += source[at++];
          }
          if (!closed) throw Error('Unterminated quote');
          continue;
        }
      }
      if (c === '#' && plural) { flush(); nodes.push(['hash']); at++; continue; }
      if (c !== '{') { text += c; at++; continue; }
      flush(); at++;
      const name = word();
      if (!/^[A-Za-z][A-Za-z0-9_]*$/.test(name)) throw Error('Invalid parameter');
      parameters.add(name);
      if (consume('}')) { nodes.push(['arg', name, 'text']); continue; }
      expect(','); const kind = word();
      if (kind === 'number') { expect('}'); nodes.push(['arg', name, kind]); continue; }
      if (!['plural', 'select'].includes(kind)) throw Error('Unsupported message format');
      expect(','); const arms = Object.create(null);
      while (!consume('}')) {
        const key = word();
        if (!key || Object.hasOwn(arms, key) || (kind === 'plural' && !/^(zero|one|two|few|many|other|=-?[0-9]+(?:\.[0-9]+)?)$/.test(key))) throw Error('Invalid choice arm');
        expect('{'); arms[key] = parse(depth + 1, plural || kind === 'plural'); expect('}');
      }
      if (!Object.hasOwn(arms, 'other')) throw Error('Choice requires other');
      nodes.push([kind, name, arms]);
    }
    flush(); return nodes;
  }
  const nodes = parse();
  if (at !== source.length) throw Error('Unexpected closing brace');
  return {nodes, parameters: [...parameters].sort()};
}

export function validatePack(pack, registry) {
  const fields = ['schemaVersion', 'locale', 'namespace', 'sourceRevision', 'messages', 'contractHash'];
  const contract=registry.metadata.pluginContracts?.[pack?.namespace];
  if (!pack || typeof pack !== 'object' || Array.isArray(pack) || Object.keys(pack).some(k => !fields.includes(k)) || pack.schemaVersion !== 1 || pack.sourceRevision !== (contract?.sourceRevision??registry.metadata.sourceRevision)) throw Error('Unsupported language package revision');
  if ((contract && pack.contractHash!==contract.contractHash) || (!contract && pack.contractHash!==undefined)) throw Error('Unknown plugin message contract');
  if (!registry.metadata.locales.some(l => l.tag === pack.locale) || typeof pack.namespace !== 'string' || !/^[a-z][a-z0-9_.]*$/.test(pack.namespace)) throw Error('Unknown locale or namespace');
  if (!pack.messages || typeof pack.messages !== 'object' || Array.isArray(pack.messages) || Object.keys(pack.messages).length > 4000) throw Error('Invalid messages');
  for (const [id, source] of Object.entries(pack.messages)) {
    const definition = registry.entries[id];
    if (!definition || definition.namespace !== pack.namespace) throw Error(`Unknown or cross-namespace message: ${id}`);
    const parse=definition.format==='interpolation.v1' ? parseInterpolation : parseMessage;
    const expected = parse(definition.messages[pack.locale] ?? definition.messages[registry.metadata.templateLocale]);
    const parsed = parse(source);
    if (JSON.stringify(parsed.parameters) !== JSON.stringify(expected.parameters)) throw Error(`Message arguments changed: ${id}`);
  }
  return pack;
}

export function parseInterpolation(source) {
  if(typeof source!=='string' || source.length>16000) throw Error('Invalid interpolated message');
  return {parameters:[...new Set([...source.matchAll(/\{(\w+)\}/g)].map(m=>m[1]))].sort()};
}
