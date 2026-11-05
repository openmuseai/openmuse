// Mock for lodash-es/debounce module
const debounce = jest.fn((func: Function, wait?: number) => {
  const debouncedFn = jest.fn((...args: any[]) => {
    return func(...args);
  });
  
  debouncedFn.cancel = jest.fn();
  debouncedFn.flush = jest.fn(() => func());
  
  return debouncedFn;
});

export default debounce;