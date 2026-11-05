import { useWriterContext } from './context';

export function useAIWriter() {
  const {
    continueWriting,
    askAIAnything,
    improveWriting,
    explain,
    makeLonger,
    makeShorter,
    fixSpelling,
    assistantType,
  } = useWriterContext();

  return {
    continueWriting,
    askAIAnything,
    improveWriting,
    explain,
    makeLonger,
    makeShorter,
    fixSpelling,
    assistantType,
  };
}