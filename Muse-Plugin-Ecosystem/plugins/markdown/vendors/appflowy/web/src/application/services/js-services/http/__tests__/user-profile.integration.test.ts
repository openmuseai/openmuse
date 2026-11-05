/**
 * @jest-environment node
 *
 * Integration tests for User Profile operations
 */

import { describe, it, expect, beforeAll, beforeEach } from '@jest/globals';
import { getEnvConfig, ensureWorkspace, AuthHelper, APIService, initAPIService } from './setup';
import { v4 as uuidv4 } from 'uuid';

describe('HTTP API - User Profile Operations', () => {
    let testWorkspaceId: string;
    let testAccessToken: string;
    let authHelper: AuthHelper;
    let mockToken: any;

    beforeAll(async () => {
        const envConfig = getEnvConfig();
        authHelper = new AuthHelper(envConfig.gotrueURL);

        initAPIService({
            baseURL: envConfig.baseURL,
            gotrueURL: envConfig.gotrueURL,
            wsURL: envConfig.wsURL,
        });

        const testEmail = `test-${uuidv4()}@appflowy.io`;

        try {
            const authResult = await authHelper.signInUser(testEmail);
            testAccessToken = authResult.accessToken;

            const expiresAt = Math.floor(Date.now() / 1000) + 3600;
            mockToken = {
                access_token: testAccessToken,
                refresh_token: authResult.refreshToken,
                expires_at: expiresAt,
                user: authResult.user,
            };

            testWorkspaceId = await ensureWorkspace(mockToken);
        } catch (error: any) {
            throw new Error(`Failed to authenticate test user: ${error.message}`);
        }
    }, 60000);

    beforeEach(() => {
        const { getTokenParsed } = require('@/application/session/token');
        getTokenParsed.mockReturnValue(mockToken);
    });

    describe('User Profile', () => {
        it('should get current user', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }
            const result = await APIService.getCurrentUser(testWorkspaceId);

            expect(result).toBeDefined();
            expect(result).toHaveProperty('uuid');
        }, 30000);

        it('should update user profile', async () => {
            await expect(
                APIService.updateUserProfile({
                    name: 'Test User Updated',
                })
            ).resolves.toBeUndefined();
        }, 30000);
    });

    describe('Workspace Member Profile', () => {
        it('should get workspace member profile', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }

            const result = await APIService.getWorkspaceMemberProfile(testWorkspaceId);

            expect(result).toBeDefined();
            expect(result).toHaveProperty('person_id');
            expect(result).toHaveProperty('name');
            expect(result).toHaveProperty('email');
        }, 30000);

        it('should update workspace member profile', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }

            await expect(
                APIService.updateWorkspaceMemberProfile(testWorkspaceId, {
                    name: 'Test Member Name',
                })
            ).resolves.toBeUndefined();
        }, 30000);
    });

    describe('User Favorites', () => {
        it('should get favorites list', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }

            const result = await APIService.getAppFavorites(testWorkspaceId);

            expect(Array.isArray(result)).toBe(true);
        }, 30000);

        it('should get recent views', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }
            const result = await APIService.getAppRecent(testWorkspaceId);

            expect(Array.isArray(result)).toBe(true);
        }, 30000);
    });
});
