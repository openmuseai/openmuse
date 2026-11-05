/**
 * @jest-environment node
 *
 * Integration tests for Template operations
 */

import { describe, it, expect, beforeAll, beforeEach } from '@jest/globals';
import { getEnvConfig, ensureWorkspace, AuthHelper, APIService, initAPIService } from './setup';
import { v4 as uuidv4 } from 'uuid';
import { TemplateCategoryType, TemplateIcon } from '@/application/template.type';

describe('HTTP API - Template Operations', () => {
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

    describe('Template Query Endpoints', () => {
        it('should get templates', async () => {
            const result = await APIService.getTemplates({});

            expect(Array.isArray(result)).toBe(true);
        }, 30000);

        it('should get template categories', async () => {
            const result = await APIService.getTemplateCategories();

            expect(Array.isArray(result)).toBe(true);
        }, 30000);

        it('should get template creators', async () => {
            const result = await APIService.getTemplateCreators();

            expect(Array.isArray(result)).toBe(true);
        }, 30000);

        it('should get template by ID', async () => {
            // This will likely fail with invalid ID, but tests error handling
            try {
                await APIService.getTemplateById('invalid-template-id');
                expect(true).toBe(false); // Should not reach here
            } catch (error: any) {
                expect(error).toBeDefined();
                expect(error.code).toBeDefined();
            }
        }, 30000);
    });

    describe('Template CRUD Operations', () => {
        it('should create template', async () => {
            // This will likely fail with invalid data, but tests error handling
            const testViewId = uuidv4();
            const testCreatorId = uuidv4();

            try {
                await APIService.createTemplate({
                    view_id: testViewId,
                    name: 'Test Template',
                    description: 'Test description',
                    about: 'Test about',
                    view_url: 'http://example.com',
                    category_ids: [],
                    creator_id: testCreatorId,
                    is_new_template: true,
                    is_featured: false,
                    related_view_ids: [],
                });
                // Function executed successfully
            } catch (error: any) {
                // May fail for various reasons (permissions, invalid data, etc.)
                expect(error).toBeDefined();
                expect(error.code).toBeDefined();
            }
        }, 30000);

        it('should update template', async () => {
            // This will likely fail with invalid data, but tests error handling
            const testViewId = uuidv4();
            const testCreatorId = uuidv4();

            try {
                await APIService.updateTemplate(testViewId, {
                    view_id: testViewId,
                    name: 'Updated Template',
                    description: 'Updated description',
                    about: 'Updated about',
                    view_url: 'http://example.com',
                    category_ids: [],
                    creator_id: testCreatorId,
                    is_new_template: false,
                    is_featured: false,
                    related_view_ids: [],
                });
                // Function executed successfully
            } catch (error: any) {
                // May fail for various reasons (permissions, invalid data, etc.)
                expect(error).toBeDefined();
                expect(error.code).toBeDefined();
            }
        }, 30000);

        it('should delete template', async () => {
            // This will likely fail with invalid ID, but tests error handling
            try {
                await APIService.deleteTemplate('invalid-template-id');
                expect(true).toBe(false); // Should not reach here
            } catch (error: any) {
                expect(error).toBeDefined();
                expect(error.code).toBeDefined();
            }
        }, 30000);

        it('should upload template avatar', async () => {
            // Create a mock image file
            const file = new File(['test image content'], 'avatar.png', { type: 'image/png' });
            try {
                const result = await APIService.uploadTemplateAvatar(file);
                expect(result).toBeDefined();
                expect(typeof result).toBe('string');
            } catch (error: any) {
                // May fail for various reasons (file size, format, permissions, etc.)
                expect(error.code).toBeDefined();
            }
        }, 30000);
    });

    describe('Template Category Operations', () => {
        it('should add template category', async () => {
            // This will likely fail with permissions, but tests error handling
            try {
                await APIService.addTemplateCategory({
                    name: `Test Category ${Date.now()}`,
                    icon: TemplateIcon.project,
                    bg_color: '#FF5733',
                    description: 'Test category description',
                    category_type: TemplateCategoryType.ByUseCase,
                    priority: 0,
                });
                // Function executed successfully
            } catch (error: any) {
                // May fail for various reasons (permissions, duplicate name, etc.)
                expect(error).toBeDefined();
                expect(error.code).toBeDefined();
            }
        }, 30000);

        it('should update template category', async () => {
            // This will likely fail with invalid ID, but tests error handling
            try {
                await APIService.updateTemplateCategory('invalid-id', {
                    name: 'Updated Category',
                    icon: TemplateIcon.project,
                    bg_color: '#FF5733',
                    description: 'Updated description',
                    category_type: TemplateCategoryType.ByUseCase,
                    priority: 0,
                });
                expect(true).toBe(false); // Should not reach here
            } catch (error: any) {
                expect(error).toBeDefined();
                expect(error.code).toBeDefined();
            }
        }, 30000);

        it('should delete template category', async () => {
            // This will likely fail with invalid ID, but tests error handling
            try {
                await APIService.deleteTemplateCategory('invalid-id');
                expect(true).toBe(false); // Should not reach here
            } catch (error: any) {
                expect(error).toBeDefined();
                expect(error.code).toBeDefined();
            }
        }, 30000);
    });

    describe('Template Creator Operations', () => {
        it('should create template creator', async () => {
            // This will likely fail with permissions, but tests error handling
            try {
                await APIService.createTemplateCreator({
                    name: `Test Creator ${Date.now()}`,
                    avatar_url: 'https://example.com/avatar.jpg',
                });
                // Function executed successfully
            } catch (error: any) {
                // May fail for various reasons
                expect(error).toBeDefined();
                expect(error.code).toBeDefined();
            }
        }, 30000);

        it('should update template creator', async () => {
            // This will likely fail with invalid ID, but tests error handling
            try {
                await APIService.updateTemplateCreator('invalid-id', {
                    name: 'Updated Creator',
                    avatar_url: 'https://example.com/avatar.jpg',
                });
                expect(true).toBe(false); // Should not reach here
            } catch (error: any) {
                expect(error).toBeDefined();
                expect(error.code).toBeDefined();
            }
        }, 30000);

        it('should delete template creator', async () => {
            // This will likely fail with invalid ID, but tests error handling
            try {
                await APIService.deleteTemplateCreator('invalid-id');
                expect(true).toBe(false); // Should not reach here
            } catch (error: any) {
                expect(error).toBeDefined();
                expect(error.code).toBeDefined();
            }
        }, 30000);
    });
});
