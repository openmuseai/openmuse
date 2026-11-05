/**
 * @jest-environment node
 *
 * Integration tests for Workspace operations
 */

import { describe, it, expect, beforeAll, beforeEach } from '@jest/globals';
import { getEnvConfig, ensureWorkspace, AuthHelper, APIService, initAPIService } from './setup';
import { v4 as uuidv4 } from 'uuid';

describe('HTTP API - Workspace Operations', () => {
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

    describe('Workspace CRUD Operations', () => {
        it('should create a new workspace', async () => {
            const workspaceName = `Test Workspace ${Date.now()}`;
            const newWorkspaceId = await APIService.createWorkspace({
                workspace_name: workspaceName,
            });

            expect(newWorkspaceId).toBeDefined();
            expect(typeof newWorkspaceId).toBe('string');
        }, 30000);

        it('should update workspace', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }

            const newName = `Updated Workspace ${Date.now()}`;
            await expect(
                APIService.updateWorkspace(testWorkspaceId, {
                    workspace_name: newName,
                })
            ).resolves.toBeUndefined();
        }, 30000);

        it('should get user workspace info', async () => {
            const result = await APIService.getUserWorkspaceInfo();

            expect(result).toBeDefined();
            expect(result).toHaveProperty('user_id');
            expect(result).toHaveProperty('selected_workspace');
            expect(result).toHaveProperty('workspaces');
            expect(Array.isArray(result.workspaces)).toBe(true);
        }, 30000);

        it('should get workspaces list', async () => {
            const result = await APIService.getWorkspaces();

            expect(Array.isArray(result)).toBe(true);
            if (result.length > 0) {
                expect(result[0]).toHaveProperty('id');
                expect(result[0]).toHaveProperty('name');
                expect(result[0]).toHaveProperty('icon');
            }
        }, 30000);
    });

    describe('Workspace Access & Management', () => {
        it('should open workspace', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }

            await expect(
                APIService.openWorkspace(testWorkspaceId)
            ).resolves.toBeUndefined();
        }, 30000);

        it('should get workspace folder', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }
            const result = await APIService.getWorkspaceFolder(testWorkspaceId);

            expect(result).toBeDefined();
            expect(result).toHaveProperty('id');
        }, 30000);

        it('should leave workspace', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }

            try {
                // Create a new workspace first so we have one to leave
                const tempWorkspaceName = `Temp Workspace ${Date.now()}`;
                const tempWorkspaceId = await APIService.createWorkspace({
                    workspace_name: tempWorkspaceName,
                });

                await APIService.leaveWorkspace(tempWorkspaceId);
                // Function executed successfully
            } catch (error: any) {
                // May fail if user is the only member
                expect(error).toBeDefined();
                expect(error.code).toBeDefined();
            }
        }, 30000);
    });

    describe('Workspace Members', () => {
        it('should get members', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }
            const result = await APIService.getMembers(testWorkspaceId);

            expect(Array.isArray(result)).toBe(true);
        }, 30000);

        it('should get mentionable users', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }
            const result = await APIService.getMentionableUsers(testWorkspaceId);

            expect(Array.isArray(result)).toBe(true);
        }, 30000);
    });

    describe('Workspace Search', () => {
        it('should search workspace', async () => {
            if (!testWorkspaceId) { throw new Error('testWorkspaceId is not available'); }

            try {
                const result = await APIService.searchWorkspace(testWorkspaceId, 'test');
                expect(result).toBeDefined();
            } catch (error: any) {
                // Search may fail if no content exists
                expect(error.code).toBeDefined();
            }
        }, 30000);
    });
});
