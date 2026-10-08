/** Bound asynchronous loader work without pretending that timeout cancels it. */
export class LifecycleWaitError extends Error {}

/**
 * The refusal `waitForLifecycle` gives a second caller while the first call's
 * work is still in flight. A distinct subclass, so a caller can tell "the same
 * operation I already know about has not finished" from a fresh failure
 * without matching on message text (#788). It stays a LifecycleWaitError, so
 * every existing `instanceof LifecycleWaitError` check keeps its meaning.
 */
export class LifecyclePendingError extends LifecycleWaitError {}

const pending = new WeakMap<object, Promise<unknown>>()
const WAIT_MS = 10_000

/** Never start a second mutation on a loader object whose earlier work is pending. */
export async function waitForLifecycle<T>(key: object, start: () => T | Promise<T>, label: string): Promise<T> {
  if (pending.has(key)) throw new LifecyclePendingError(`${label}: previous operation is still pending`)
  const operation = Promise.resolve().then(start)
  pending.set(key, operation)
  // Both handlers remain attached after timeout, including for late rejection.
  void operation.then(
    () => { if (pending.get(key) === operation) pending.delete(key) },
    () => { if (pending.get(key) === operation) pending.delete(key) },
  )
  let timer: ReturnType<typeof setTimeout> | undefined
  try {
    return await Promise.race([
      operation,
      new Promise<never>((_resolve, reject) => {
        timer = setTimeout(() => reject(new LifecycleWaitError(`${label}: did not settle within ${WAIT_MS / 1000}s; runtime state is uncertain`)), WAIT_MS)
      }),
    ])
  } finally { clearTimeout(timer) }
}
