import { EditorData, EditorNode } from '@appflowyinc/editor';

interface Element {
  type: string;
  data?: Record<string, unknown>;
  children: (Element | Text)[];
}

interface Text extends Record<string, unknown> {
  text: string;
}

export function convertToAppFlowyFragment(data: EditorData) {
  const key = 'application/x-appflowy-fragment';

  const traverse = (item: EditorNode) => {
    const data = item.data || {};
    const delta = item.delta;

    const textNode: Element | undefined = delta ? {
      type: 'text',
      children: delta.map(op => ({
        text: op.insert,
        ...op.attributes,
      })) as Text[],
    } : undefined;

    const children = item.children.map(traverse);

    if(textNode) {
      children.unshift(textNode);
    }

    const newNode: Element = {
      type: item.type,
      data,
      children,
    };

    return newNode;
  };

  const fragment = data.map(traverse);
  const string = JSON.stringify(fragment);
  const encoded = window.btoa(encodeURIComponent(string));

  return {
    key,
    value: encoded,
  };
}