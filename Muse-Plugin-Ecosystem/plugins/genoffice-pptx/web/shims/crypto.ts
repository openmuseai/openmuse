import { sha256 } from '@noble/hashes/sha2.js'
import { Buffer } from 'buffer'
export const randomUUID = () => globalThis.crypto.randomUUID()
export function createHash(algorithm: string) {
  if (algorithm !== 'sha256') throw new Error(`Unsupported digest: ${algorithm}`)
  const hash = sha256.create()
  return {
    update(bytes: Uint8Array | string) { hash.update(typeof bytes === 'string' ? new TextEncoder().encode(bytes) : bytes); return this },
    digest(encoding?: string) { const bytes = hash.digest(); return encoding === 'hex' ? Buffer.from(bytes).toString('hex') : Buffer.from(bytes) },
  }
}
