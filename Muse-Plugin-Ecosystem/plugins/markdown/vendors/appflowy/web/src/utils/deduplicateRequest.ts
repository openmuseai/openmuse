// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function createDeduplicatedRequest<T extends (...args: any[]) => Promise<any>>(
  requestFn: T,
  keyGenerator?: (...args: Parameters<T>) => string
): T {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const pendingRequests = new Map<string, Promise<any>>();

  return ((...args: Parameters<T>) => {
    const key = keyGenerator ? keyGenerator(...args) : JSON.stringify(args);

    if (pendingRequests.has(key)) {
      return pendingRequests.get(key)!;
    }

    const promise = requestFn(...args).finally(() => {
      pendingRequests.delete(key);
    });

    pendingRequests.set(key, promise);

    return promise;
  }) as T;
}

export function createDeduplicatedNoArgsRequest<T>(requestFn: () => Promise<T>): () => Promise<T> {
  let pendingRequest: Promise<T> | null = null;

  return () => {
    if (pendingRequest) {
      return pendingRequest;
    }

    pendingRequest = requestFn().finally(() => {
      pendingRequest = null;
    });

    return pendingRequest;
  };
}
