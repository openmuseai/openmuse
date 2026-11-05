import { Element, Text } from 'slate';
import { ReactEditor } from 'slate-react';

import { BlockType } from '@/application/types';
import { HeadingNode } from '@/components/editor/editor.type';

import { CustomEditor } from 'src/application/slate-yjs/command';

export function extractHeadings(editor: ReactEditor, maxDepth: number): {
  hasHeadings: boolean;
  headings: HeadingNode[];
} {
  const headings: HeadingNode[] = [];
  const blocks = editor.children;
  let hasHeadings = false;

  function traverse(children: (Element | Text)[]) {
    for (const block of children) {
      if (Text.isText(block)) continue;

      const isHeading = [
        BlockType.HeadingBlock,
        BlockType.ToggleListBlock,
      ].includes(block.type as BlockType);

      if (isHeading) {
        hasHeadings = true;
      }

      if (isHeading && 'level' in (block as HeadingNode).data && (block as HeadingNode).data?.level <= maxDepth) {
        headings.push({
          ...block,
          data: {
            level: (block as HeadingNode).data.level,
            text: CustomEditor.getBlockTextContent(block, 2).trim(),
          },
          children: [],
        } as HeadingNode);
      } else {
        traverse(block.children);
      }
    }
  }

  traverse(blocks);

  return {
    hasHeadings,
    headings,
  };
}

export function nestHeadings(headings: HeadingNode[]): HeadingNode[] {
  const root: HeadingNode[] = [];
  const stack: HeadingNode[] = [];

  headings.forEach((heading) => {
    const node = { ...heading, children: [] };

    while (stack.length > 0 && stack[stack.length - 1].data.level >= node.data.level) {
      stack.pop();
    }

    if (stack.length === 0) {
      root.push(node);
    } else {
      stack[stack.length - 1].children.push(node);
    }

    stack.push(node);
  });

  return root;
}
