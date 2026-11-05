export enum AiPromptCategory {
  Development = 'development',
  Writing = 'writing',
  HealthAndFitness = 'healthAndFitness',
  Business = 'business',
  Marketing = 'marketing',
  Travel = 'travel',
  Others = 'others',
  ContentSeo = 'contentSeo',
  EmailMarketing = 'emailMarketing',
  PaidAds = 'paidAds',
  PrCommunication = 'prCommunication',
  Recruiting = 'recruiting',
  Sales = 'sales',
  SocialMedia = 'socialMedia',
  Strategy = 'strategy',
  CaseStudies = 'caseStudies',
  SalesCopy = 'salesCopy',
  Education = 'education',
  Work = 'work',
  PodcastProduction = 'podcastProduction',
  CopyWriting = 'copyWriting',
  CustomerSuccess = 'customerSuccess',
}

export interface AiPrompt {
  id: string;
  name: string;
  category: AiPromptCategory[];
  content: string;
  example: string;
  isFeatured: boolean;
  isCustom: boolean;
}

export interface RawPromptData {
  id: string;
  name: string;
  category?: string;
  content: string;
  example?: string;
  isFeatured?: boolean;
  isCustom?: boolean;
}

export interface PromptDatabaseField {
  id: string;
  name: string;
  isPrimary: boolean;
  isSelect: boolean;
}
