import { readFile } from 'node:fs/promises'
import { createHash } from 'node:crypto'
import { openPptx, savePptxToFile } from '@genoffice/pptx-engine'

const MAX_BYTES = 64 * 1024 * 1024

export async function inspect(file: string) {
  const bytes = await readFile(file)
  return inspectBytes(bytes)
}

export async function inspectBytes(bytes: Uint8Array) {
  if (bytes.length === 0 || bytes.length > MAX_BYTES) {
    throw new Error('PPTX size exceeds plugin limit')
  }
  const opened = await openPptx(bytes)
  return {
    slides: opened.deck.slides.length,
    size: opened.deck.size,
    sha256: createHash('sha256').update(bytes).digest('hex'),
    pages: opened.deck.slides.map((slide, index) => ({
      index,
      elements: slide.elements.length,
    })),
  }
}

export async function roundtrip(file: string, out: string) {
  const bytes = await readFile(file)
  if (bytes.length === 0 || bytes.length > MAX_BYTES) {
    throw new Error('PPTX size exceeds plugin limit')
  }
  const opened = await openPptx(bytes)
  await savePptxToFile(opened, out)
  const saved = await readFile(out)
  const reopened = await openPptx(saved)
  return {
    byteIdentical: Buffer.compare(bytes, Buffer.from(saved)) === 0,
    slidesPreserved: reopened.deck.slides.length === opened.deck.slides.length,
    sha256: createHash('sha256').update(saved).digest('hex'),
  }
}

if (process.argv[1]?.endsWith('/worker.ts') || process.argv[1]?.endsWith('/worker.mjs')) {
  const [operation, file, out] = process.argv.slice(2)
  try {
    if (!file) throw new Error('PPTX path required')
    const result = operation === 'inspect'
      ? await inspect(file)
      : operation === 'roundtrip' && out
        ? await roundtrip(file, out)
        : (() => { throw new Error('unsupported operation') })()
    process.stdout.write(`${JSON.stringify(result)}\n`)
  } catch (error) {
    process.stderr.write(`${error}\n`)
    process.exitCode = 1
  }
}
