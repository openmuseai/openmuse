import { BaseRange, Editor, Element, Node, Text } from 'slate';

import { AwarenessSelection } from './types';

// Extended element types
interface TextElement extends Element {
  textId?: string;
}

/**
 * Convert block offset to inline node path and offset
 */
function convertBlockOffsetToInlineOffset(
  textNode: Element,
  basePath: number[],
  blockOffset: number
): { path: number[]; offset: number } {
  if (!textNode.children) {
    return { path: basePath, offset: blockOffset };
  }

  let currentOffset = 0;

  for (const [i, child] of textNode.children.entries()) {
    if (Text.isText(child)) {
      const textLength = child.text.length;

      // If offset is within current text node range
      if (blockOffset <= currentOffset + textLength) {
        return {
          path: [...basePath, i],
          offset: blockOffset - currentOffset,
        };
      }

      currentOffset += textLength;
    } else if (Element.isElement(child)) {
      // For inline elements, calculate their text length
      const elementTextLength = getTextLength(child);

      // If offset is within current element range
      if (blockOffset <= currentOffset + elementTextLength) {
        // Recursively calculate offset within element
        const relativeOffset = blockOffset - currentOffset;

        return calculateOffsetInElement(child, [...basePath, i], relativeOffset);
      }

      currentOffset += elementTextLength;
    }
  }

  // If offset exceeds range, return the end of last node
  if (textNode.children.length > 0) {
    const lastChild = textNode.children[textNode.children.length - 1];

    if (Text.isText(lastChild)) {
      return {
        path: [...basePath, textNode.children.length - 1],
        offset: lastChild.text.length,
      };
    }
  }

  // Default return original path and offset
  return {
    path: basePath,
    offset: blockOffset,
  };
}

/**
 * Get the total text length of an element (including all nested text nodes)
 */
function getTextLength(element: Element): number {
  let totalLength = 0;

  if (element.children) {
    for (const child of element.children) {
      if (Text.isText(child)) {
        totalLength += child.text.length;
      } else if (Element.isElement(child)) {
        totalLength += getTextLength(child);
      }
    }
  }

  return totalLength;
}

/**
 * Calculate offset within an element
 */
function calculateOffsetInElement(
  element: Element,
  basePath: number[],
  targetOffset: number
): { path: number[]; offset: number } {
  let currentOffset = 0;

  if (element.children) {
    for (const [i, child] of element.children.entries()) {
      if (Text.isText(child)) {
        const textLength = child.text.length;

        if (targetOffset <= currentOffset + textLength) {
          return {
            path: [...basePath, i],
            offset: targetOffset - currentOffset,
          };
        }

        currentOffset += textLength;
      } else if (Element.isElement(child)) {
        const elementTextLength = getTextLength(child);

        if (targetOffset <= currentOffset + elementTextLength) {
          const relativeOffset = targetOffset - currentOffset;

          return calculateOffsetInElement(child, [...basePath, i], relativeOffset);
        }

        currentOffset += elementTextLength;
      }
    }
  }

  // If no suitable position found, return element end
  return {
    path: basePath,
    offset: targetOffset,
  };
}

/**
 * Calculate the actual path and offset for a given slate path and awareness offset
 */
function calculateActualPathAndOffset(
  children: Node[],
  slatePath: number[],
  awarenessOffset: number
): { path: number[]; offset: number } {
  try {
    // Find target node through path
    let currentNode: Node | Node[] = children;

    // Traverse path to find target node
    for (const index of slatePath) {
      if (Array.isArray(currentNode)) {
        currentNode = currentNode[index];
      } else if (Element.isElement(currentNode) && currentNode.children) {
        currentNode = currentNode.children[index];
      } else {
        break;
      }
    }

    // If target is a text node, use specialized function to handle block offset conversion
    if (Element.isElement(currentNode) && currentNode.children) {
      return convertBlockOffsetToInlineOffset(currentNode, slatePath, awarenessOffset);
    }

    // Default return original path and offset
    return {
      path: slatePath,
      offset: awarenessOffset,
    };
  } catch (error) {
    return {
      path: slatePath,
      offset: awarenessOffset,
    };
  }
}

/**
 * Convert AwarenessSelection to Slate-compatible BaseRange
 *
 * AwarenessSelection paths ignore text nodes, need to correctly convert to Slate paths
 * Examples:
 * - AwarenessSelection [0] -> Slate [0, 0] (text node of block 0)
 * - AwarenessSelection [0, 0] -> Slate [0, 1, 0] (text node of first child of block 0)
 * - AwarenessSelection [0, 1] -> Slate [0, 2, 0] (text node of second child of block 0)
 */
export function convertAwarenessSelection(selection: AwarenessSelection, children: Node[]): BaseRange {
  const convertToSlatePath = (awarenessPath: number[]): number[] => {
    if (awarenessPath.length === 0) return [];

    const blockIndex = awarenessPath[0];

    // If only one level path, it points to block level, corresponding to text node
    if (awarenessPath.length === 1) {
      return [blockIndex, 0];
    }

    // Multi-level paths need to consider textId offset at each level
    const slatePath = [blockIndex];
    let currentNode: Node | undefined;

    // Get target block node
    if (children && children[blockIndex]) {
      currentNode = children[blockIndex];

      // Process subsequent paths, check textId offset at each level
      for (let i = 1; i < awarenessPath.length; i++) {
        const awarenessIndex = awarenessPath[i];

        // Check if current level has textId
        const hasTextId =
          Element.isElement(currentNode) &&
          currentNode.children &&
          currentNode.children[0] &&
          Element.isElement(currentNode.children[0]) &&
          (currentNode.children[0] as TextElement).textId;

        if (hasTextId) {
          // If has textId, AwarenessSelection index needs +1
          slatePath.push(awarenessIndex + 1);

          // Update currentNode to next level for checking
          if (Element.isElement(currentNode) && currentNode.children && currentNode.children[awarenessIndex + 1]) {
            currentNode = currentNode.children[awarenessIndex + 1];
          }
        } else {
          // If no textId, directly use awareness index
          slatePath.push(awarenessIndex);

          // Update currentNode to next level for checking
          if (Element.isElement(currentNode) && currentNode.children && currentNode.children[awarenessIndex]) {
            currentNode = currentNode.children[awarenessIndex];
          }
        }
      }
    }

    // Append text path, text path index is 0
    slatePath.push(0);

    return slatePath;
  };

  // Convert start and end paths
  const startSlatePath = convertToSlatePath(selection.start.path);
  const endSlatePath = convertToSlatePath(selection.end.path);

  // Calculate actual paths and offsets
  const startActual = calculateActualPathAndOffset(children, startSlatePath, selection.start.offset);
  const endActual = calculateActualPathAndOffset(children, endSlatePath, selection.end.offset);

  // Return BaseRange directly
  return {
    anchor: {
      path: startActual.path,
      offset: startActual.offset,
    },
    focus: {
      path: endActual.path,
      offset: endActual.offset,
    },
  };
}

/**
 * Convert Slate path to AwarenessSelection path
 *
 * Slate paths always end with text node index, which should be removed in AwarenessSelection
 * Examples:
 * - Slate [0, 0] -> Awareness [0] (remove text node index 0)
 * - Slate [0, 1, 0] -> Awareness [0, 0] (remove text node index 0, adjust textId offset: 1-1=0)
 * - Slate [0, 2, 0] -> Awareness [0, 1] (remove text node index 0, adjust textId offset: 2-1=1)
 */
function convertSlatePathToAwarenessPath(children: Node[], slatePath: number[]): number[] {
  if (slatePath.length === 0) return [];

  const blockIndex = slatePath[0];

  // If path only points to text node level [blockIndex, 0], convert to block level
  if (slatePath.length === 2 && slatePath[1] === 0) {
    return [blockIndex];
  }

  // For deeper paths, remove the last element (text node index) and process the rest
  const awarenessPath = [blockIndex];
  let currentNode: Node | Node[] = children;

  // Navigate to block node
  if (Array.isArray(currentNode) && currentNode[blockIndex]) {
    currentNode = currentNode[blockIndex];
  }

  // Process each level of the path (excluding the last text node index)
  // slatePath.length - 1 to exclude the text node index
  for (let i = 1; i < slatePath.length - 1; i++) {
    const slateIndex = slatePath[i];

    // Check if current level has textId offset
    const hasTextId =
      Element.isElement(currentNode) &&
      currentNode.children &&
      currentNode.children[0] &&
      Element.isElement(currentNode.children[0]) &&
      (currentNode.children[0] as TextElement).textId;

    if (hasTextId) {
      // If has textId, subtract 1 from slate index to get awareness index
      const awarenessIndex = slateIndex - 1;

      // Only push valid indices (>= 0)
      if (awarenessIndex >= 0) {
        awarenessPath.push(awarenessIndex);
      }

      // Navigate to next level
      if (Element.isElement(currentNode) && currentNode.children && currentNode.children[slateIndex]) {
        currentNode = currentNode.children[slateIndex];
      }
    } else {
      // If no textId, directly use slate index
      awarenessPath.push(slateIndex);

      // Navigate to next level
      if (Element.isElement(currentNode) && currentNode.children && currentNode.children[slateIndex]) {
        currentNode = currentNode.children[slateIndex];
      }
    }
  }

  return awarenessPath;
}

/**
 * Convert Slate BaseRange to AwarenessSelection
 *
 * This is the reverse function of convertAwarenessSelection
 * Examples:
 * - Slate [0, 0] -> AwarenessSelection [0] (block 0)
 * - Slate [0, 1, 0] -> AwarenessSelection [0, 0] (first child of block 0)
 * - Slate [0, 2, 0] -> AwarenessSelection [0, 1] (second child of block 0)
 */
/**
 * Convert Slate BaseRange point to awareness block offset
 * Uses editor utilities to calculate the offset from block start to the target point
 * @param editor - The slate editor instance
 * @param slatePoint - Slate point (path + offset)
 * @returns Offset from the beginning of the awareness block
 */
function convertSlatePointToAwarenessBlockOffset(
  editor: Editor,
  slatePoint: { path: number[]; offset: number }
): number {
  try {
    // Get the block entry for this point
    const blockEntry = Editor.above(editor, {
      at: slatePoint,
      match: (n: Node) => !Editor.isEditor(n) && Element.isElement(n) && n.blockId !== undefined,
    });

    if (!blockEntry) {
      return slatePoint.offset;
    }

    const [, blockPath] = blockEntry;

    // Get the start point of the block
    const blockStart = Editor.start(editor, blockPath);

    // Calculate string length from block start to the target point
    const stringLength = Editor.string(editor, {
      anchor: blockStart,
      focus: slatePoint,
    }).length;

    return stringLength;
  } catch (error) {
    return slatePoint.offset;
  }
}

export function convertSlateSelectionToAwareness(baseRange: BaseRange, editor: Editor): AwarenessSelection {
  // Convert anchor
  const startAwarenessPath = convertSlatePathToAwarenessPath(editor.children, baseRange.anchor.path);
  const startBlockOffset = convertSlatePointToAwarenessBlockOffset(editor, baseRange.anchor);

  // Convert focus
  const endAwarenessPath = convertSlatePathToAwarenessPath(editor.children, baseRange.focus.path);
  const endBlockOffset = convertSlatePointToAwarenessBlockOffset(editor, baseRange.focus);

  return {
    start: {
      path: startAwarenessPath,
      offset: startBlockOffset,
    },
    end: {
      path: endAwarenessPath,
      offset: endBlockOffset,
    },
  };
}

function hashUsername(username: string) {
  let hash = 0;

  for (let i = 0; i < username.length; i++) {
    const char = username.charCodeAt(i);

    hash = (hash << 5) - hash + char;
    hash = hash & hash; // Convert to 32-bit integer
  }

  return Math.abs(hash) % 20;
}

/**
 * Generate random color for user cursor and selection
 * @param username - User name string
 * @returns Object with cursor_color and selection_color using badge color system
 */
export function generateUserColors(username: string) {
  // Badge color pairs from design system CSS variables
  // Each pair contains: [cursor_color (thick-2), selection_color (light-2)]
  const colorPairs = [
    ['#cc4e4e', '#fae3e3'], // badge-color-1
    ['#d67240', '#fae8de'], // badge-color-2
    ['#db8f2c', '#fcedd9'], // badge-color-3
    ['#e0a416', '#fcf1d7'], // badge-color-4
    ['#e0bb00', '#fcf5cf'], // badge-color-5
    ['#adb204', '#f6f7d0'], // badge-color-6
    ['#92a822', '#eef5ce'], // badge-color-7
    ['#75a828', '#e9f5d7'], // badge-color-8
    ['#49a33b', '#e2f5df'], // badge-color-9
    ['#1c9963', '#dff5eb'], // badge-color-10
    ['#008e9e', '#dff3f5'], // badge-color-11
    ['#0877cc', '#e1eef7'], // badge-color-12
    ['#3267d1', '#e3ebfa'], // badge-color-13
    ['#5555e0', '#e6e6fa'], // badge-color-14
    ['#8153db', '#ebe3fa'], // badge-color-15
    ['#9e4cc7', '#f0e1f7'], // badge-color-16
    ['#b240af', '#f5e1f4'], // badge-color-17
    ['#c24279', '#f7e1eb'], // badge-color-18
    ['#6e6e6e', '#e8e8e8'], // badge-color-19
    ['#666f80', '#e6e9f0'], // badge-color-20
  ];

  const hash = hashUsername(username);
  const [cursorColor, selectionColor] = colorPairs[hash];

  return {
    cursor_color: cursorColor,
    selection_color: selectionColor,
  };
}
