export function getToolPosition(writer: HTMLElement) {
  const writerRect = writer.getBoundingClientRect();

  return {
    top: writerRect.top + writerRect.height,
    left: writerRect.left,
  };
}