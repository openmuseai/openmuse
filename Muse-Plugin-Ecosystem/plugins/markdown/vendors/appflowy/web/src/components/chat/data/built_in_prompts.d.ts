declare module '../data/prompts.json' {
  interface PromptsJsonContent {
    prompts: Array<{
      id: string;
      name: string;
      category?: string;
      content: string;
      example?: string;
      isFeatured?: boolean;
      isCustom?: boolean;
    }>;
  }
  const value: PromptsJsonContent;
  export default value;
}
