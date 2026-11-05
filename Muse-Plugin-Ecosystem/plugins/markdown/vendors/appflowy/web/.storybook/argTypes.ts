/**
 * Shared argTypes for Storybook stories
 *
 * This file contains common argTypes definitions to avoid duplication across story files.
 * Import and spread these into your story's argTypes.
 */

import { SubscriptionPlan } from '@/application/types';

/**
 * ArgType for hostname control
 * Use this for stories that need to test different hosting scenarios
 */
export const hostnameArgType = {
  hostname: {
    control: 'text',
    description: 'Mock hostname to simulate different hosting scenarios (e.g., "beta.appflowy.cloud" for official host, "self-hosted.example.com" for self-hosted)',
    table: {
      category: 'Testing',
      defaultValue: { summary: 'beta.appflowy.cloud' },
    },
  },
};

/**
 * ArgType for active subscription plan
 * Use this for stories that need to test different subscription states
 */
export const subscriptionPlanArgType = {
  activeSubscriptionPlan: {
    control: 'select',
    options: [SubscriptionPlan.Free, SubscriptionPlan.Pro, null],
    description: 'Current subscription plan of the user',
    table: {
      category: 'Subscription',
      type: { summary: 'SubscriptionPlan | null' },
      defaultValue: { summary: 'null' },
    },
  },
};

/**
 * ArgType for active plan (shorter name, same as above)
 * Alias for subscriptionPlanArgType
 */
export const activePlanArgType = {
  activePlan: {
    control: 'select',
    options: [SubscriptionPlan.Free, SubscriptionPlan.Pro, null],
    description: 'Current subscription plan of the user',
    table: {
      category: 'Subscription',
      type: { summary: 'SubscriptionPlan | null' },
      defaultValue: { summary: 'null' },
    },
  },
};

/**
 * ArgType for isOwner flag
 * Use this for stories that test owner vs non-owner behavior
 */
export const isOwnerArgType = {
  isOwner: {
    control: 'boolean',
    description: 'Whether the current user is the workspace owner',
    table: {
      category: 'User',
      type: { summary: 'boolean' },
      defaultValue: { summary: 'true' },
    },
  },
};

/**
 * ArgType for modal open state
 * Use this for stories with modal/dialog components
 */
export const openArgType = {
  open: {
    control: 'boolean',
    description: 'Whether the modal/dialog is open',
    table: {
      category: 'State',
      type: { summary: 'boolean' },
      defaultValue: { summary: 'false' },
    },
  },
};

/**
 * Combined argTypes for hostname-aware subscription components
 * Common pattern for components that check both hostname and subscription
 */
export const hostnameAndSubscriptionArgTypes = {
  ...hostnameArgType,
  ...subscriptionPlanArgType,
};

/**
 * Combined argTypes for ownership-aware components
 */
export const ownershipArgTypes = {
  ...isOwnerArgType,
  ...subscriptionPlanArgType,
};
