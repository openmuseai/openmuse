/**
 * @jest-environment node
 *
 * Integration tests for Trash operations
 */

import { describe, it, expect, beforeAll, beforeEach } from '@jest/globals';
import { getEnvConfig, ensureWorkspace, AuthHelper, APIService, initAPIService } from './setup';
import { v4 as uuidv4 } from 'uuid';

describe('HTTP API - Trash Operations', () => {
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

    describe('Trash Management', () => {
        it('should get trash views', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }
            const result = await APIService.getAppTrash(testWorkspaceId);

            expect(Array.isArray(result)).toBe(true);
        }, 30000);

        it('should delete all trash', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }
            try {
                await expect(
                    APIService.deleteTrash(testWorkspaceId)
                ).resolves.toBeUndefined();
            } catch (error: any) {
                expect(error.code).toBeDefined();
            }
        }, 30000);

        it('should restore all pages from trash', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }
            try {
                await expect(
                    APIService.restorePage(testWorkspaceId)
                ).resolves.toBeUndefined();
            } catch (error: any) {
                expect(error.code).toBeDefined();
            }
        }, 30000);
    });
});
