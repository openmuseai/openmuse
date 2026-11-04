import { vi } from 'vitest'

// `marketFetch` calls undici's fetch with its own dispatcher (#742). The
// unit lane stubs the global fetch, not the package. Forward the package
// fetch onto that stub and drop the dispatcher, so a stub written against
// `init.headers` / `init.signal` still sees the call it was written for.
vi.mock('undici', async (importOriginal) => {
  const actual = await importOriginal<typeof import('undici')>()
  return {
    ...actual,
    fetch: (input: RequestInfo | URL, init?: RequestInit & { dispatcher?: unknown }) => {
      if (init === undefined) return globalThis.fetch(input)
      const { dispatcher: _dispatcher, ...rest } = init
      return globalThis.fetch(input, rest)
    },
  }
})
