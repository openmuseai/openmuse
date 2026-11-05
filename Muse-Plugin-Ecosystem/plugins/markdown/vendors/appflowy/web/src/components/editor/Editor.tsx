import { memo, useCallback, useState } from 'react';

import { YjsEditor } from '@/application/slate-yjs';
import { YDoc } from '@/application/types';
import CollaborativeEditor from '@/components/editor/CollaborativeEditor';
import { defaultLayoutStyle, EditorContextProvider, EditorContextState } from '@/components/editor/EditorContext';
import './editor.scss';

export interface EditorProps extends EditorContextState {
  doc: YDoc;
  onEditorConnected?: (editor: YjsEditor) => void;
  onSelectionChange?: (editor: YjsEditor) => void;
}

export const Editor = memo(
  ({ doc, onEditorConnected, onSelectionChange, layoutStyle = defaultLayoutStyle, ...props }: EditorProps) => {
    const [codeGrammars, setCodeGrammars] = useState<Record<string, string>>({});

    const handleAddCodeGrammars = useCallback((blockId: string, grammar: string) => {
      setCodeGrammars((prev) => ({ ...prev, [blockId]: grammar }));
    }, []);

    return (
      <EditorContextProvider
        {...props}
        codeGrammars={codeGrammars}
        addCodeGrammars={handleAddCodeGrammars}
        layoutStyle={layoutStyle}
      >
        <CollaborativeEditor doc={doc} onEditorConnected={onEditorConnected} onSelectionChange={onSelectionChange} />
      </EditorContextProvider>
    );
  }
);

export default Editor;
