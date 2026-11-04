export const join = (...parts: string[]) => parts.join('/').replace(/\/{2,}/g,'/')
