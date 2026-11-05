// Mock for lodash-es/isEqual module
const isEqual = jest.fn((a: any, b: any) => {
  return true;
});

export default isEqual;
