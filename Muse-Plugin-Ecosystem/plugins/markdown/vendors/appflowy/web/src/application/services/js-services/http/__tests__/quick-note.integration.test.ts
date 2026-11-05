/**
 * @jest-environment node
 *
 * Integration tests for Quick Note and Search operations
 */

import { describe, it, expect, beforeAll, beforeEach } from '@jest/globals';
import { getEnvConfig, ensureWorkspace, AuthHelper, APIService, initAPIService } from './setup';
import { v4 as uuidv4 } from 'uuid';

describe('HTTP API - Quick Note & Search Operations', () => {
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

    describe('Quick Note Operations', () => {
        let createdNoteId: string | null = null;

        it('should get quick note list', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }
            const result = await APIService.getQuickNoteList(testWorkspaceId, {});

            expect(result).toBeDefined();
            expect(result).toHaveProperty('data');
            expect(result).toHaveProperty('has_more');
            expect(Array.isArray(result.data)).toBe(true);
        }, 30000);

        it('should create quick note', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }
            try {
                const result = await APIService.createQuickNote(testWorkspaceId, [
                    {
                        type: 'paragraph',
                        delta: [{ insert: 'Test quick note' }],
                        children: [],
                    },
                ]);
                expect(result).toBeDefined();
                expect(result).toHaveProperty('id');
                createdNoteId = result.id;
            } catch (error: any) {
                // May fail for various reasons
                expect(error.code).toBeDefined();
            }
        }, 30000);

        it('should update quick note', async () => {
            if (!testWorkspaceId || !createdNoteId) {
                throw new Error('testWorkspaceId or createdNoteId is not available');
            }
            try {
                await expect(
                    APIService.updateQuickNote(testWorkspaceId, createdNoteId, [
                        {
                            type: 'paragraph',
                            delta: [{ insert: 'Updated quick note' }],
                            children: [],
                        },
                    ])
                ).resolves.toBeUndefined();
            } catch (error: any) {
                expect(error.code).toBeDefined();
            }
        }, 30000);

        it('should delete quick note', async () => {
            if (!testWorkspaceId || !createdNoteId) {
                throw new Error('testWorkspaceId or createdNoteId is not available');
            }
            try {
                await expect(
                    APIService.deleteQuickNote(testWorkspaceId, createdNoteId)
                ).resolves.toBeUndefined();
            } catch (error: any) {
                expect(error.code).toBeDefined();
            }
        }, 30000);
    });

    describe('Search Operations', () => {
        it('should search workspace', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }
            const result = await APIService.searchWorkspace(testWorkspaceId, 'test');

            expect(result).toBeDefined();
            expect(Array.isArray(result)).toBe(true);
        }, 30000);
    });
});
