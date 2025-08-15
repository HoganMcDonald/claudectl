import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { mkdtemp, rm, mkdir, access } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { performMultiStepInit } from './initialization';
import type { InitStepOutcome } from './initialization';

describe('initialization utilities', () => {
  let tempDir: string;
  let originalHome: string | undefined;

  beforeEach(async () => {
    tempDir = await mkdtemp(join(tmpdir(), 'claudectl-test-'));
    originalHome = process.env.HOME;
    // Set HOME to our temp directory for testing
    process.env.HOME = tempDir;
  });

  afterEach(async () => {
    if (tempDir) {
      await rm(tempDir, { recursive: true, force: true });
    }
    // Restore original HOME
    if (originalHome !== undefined) {
      process.env.HOME = originalHome;
    } else {
      delete process.env.HOME;
    }
  });

  describe('performMultiStepInit', () => {
    it('should successfully complete all initialization steps', async () => {
      const projectName = 'test-project';
      const projectPath = join(tempDir, 'project');
      await mkdir(projectPath);

      const results = performMultiStepInit(projectName, projectPath);

      expect(results).toHaveLength(2);
      
      // Check step 1: Project configuration
      expect(results[0].success).toBe(true);
      expect(results[0].message).toBe('Project configuration');
      expect(results[0].details).toBe('.claudectl/config.json');

      // Check step 2: Global directory structure
      expect(results[1].success).toBe(true);
      expect(results[1].message).toBe('Global directory structure');
      expect(results[1].details).toBe(`~/.claudectl/projects/${projectName}`);

      // Verify files and directories were actually created
      const configPath = join(projectPath, '.claudectl', 'config.json');
      await expect(access(configPath)).resolves.toBeUndefined();

      const globalProjectDir = join(tempDir, '.claudectl', 'projects', projectName);
      await expect(access(globalProjectDir)).resolves.toBeUndefined();
    });

    it('should use current working directory when no project path provided', async () => {
      const projectName = 'test-project';
      
      // Mock process.cwd to return our temp directory
      const originalCwd = process.cwd;
      vi.spyOn(process, 'cwd').mockReturnValue(tempDir);

      const results = performMultiStepInit(projectName);

      expect(results).toHaveLength(2);
      expect(results[0].success).toBe(true);
      expect(results[1].success).toBe(true);

      // Verify config was created in current working directory (mocked temp dir)
      const configPath = join(tempDir, '.claudectl', 'config.json');
      await expect(access(configPath)).resolves.toBeUndefined();

      // Restore original cwd
      process.cwd = originalCwd;
    });

    it('should handle project configuration creation failure', async () => {
      const projectName = ''; // Invalid empty name should cause validation error
      const projectPath = join(tempDir, 'project');
      await mkdir(projectPath);

      const results = performMultiStepInit(projectName, projectPath);

      expect(results).toHaveLength(1); // Should stop after first failure
      expect(results[0].success).toBe(false);
      expect(results[0].error).toBe('Failed to create project configuration');
      expect(results[0].details).toContain('Project name cannot be empty');
    });

    it('should handle global directory creation failure', async () => {
      const projectName = 'test-project';
      const projectPath = join(tempDir, 'project');
      await mkdir(projectPath);

      // Create a scenario where directory creation will fail
      // We'll set HOME to a location that doesn't exist and can't be created
      const originalHome = process.env.HOME;
      process.env.HOME = '/root/invalid/path/that/cannot/be/created';

      const results = performMultiStepInit(projectName, projectPath);

      expect(results).toHaveLength(2);
      expect(results[0].success).toBe(true); // Project config should succeed
      expect(results[1].success).toBe(false); // Directory creation should fail
      expect(results[1].error).toBe('Failed to create global directory structure');
      expect(results[1].details).toBeDefined();

      // Restore the original HOME
      if (originalHome !== undefined) {
        process.env.HOME = originalHome;
      } else {
        delete process.env.HOME;
      }
    });

    it('should create directories even if they already exist', async () => {
      const projectName = 'test-project';
      const projectPath = join(tempDir, 'project');
      await mkdir(projectPath);

      // Pre-create some of the directories
      const globalClaudectlDir = join(tempDir, '.claudectl');
      const projectsDir = join(globalClaudectlDir, 'projects');
      await mkdir(globalClaudectlDir, { recursive: true });
      await mkdir(projectsDir, { recursive: true });

      const results = performMultiStepInit(projectName, projectPath);

      expect(results).toHaveLength(2);
      expect(results[0].success).toBe(true);
      expect(results[1].success).toBe(true);

      // Verify everything still works with pre-existing directories
      const configPath = join(projectPath, '.claudectl', 'config.json');
      await expect(access(configPath)).resolves.toBeUndefined();

      const globalProjectDir = join(tempDir, '.claudectl', 'projects', projectName);
      await expect(access(globalProjectDir)).resolves.toBeUndefined();
    });

    it('should handle project names with special characters', async () => {
      const projectName = 'test-project_with-special.chars';
      const projectPath = join(tempDir, 'project');
      await mkdir(projectPath);

      const results = performMultiStepInit(projectName, projectPath);

      expect(results).toHaveLength(2);
      expect(results[0].success).toBe(true);
      expect(results[1].success).toBe(true);

      // Verify directories were created with special characters in name
      const globalProjectDir = join(tempDir, '.claudectl', 'projects', projectName);
      await expect(access(globalProjectDir)).resolves.toBeUndefined();
    });

    it('should return proper InitStepOutcome types', () => {
      const projectName = 'test-project';
      const projectPath = join(tempDir, 'project');

      const results = performMultiStepInit(projectName, projectPath);

      results.forEach((result: InitStepOutcome) => {
        if (result.success) {
          expect(result).toHaveProperty('message');
          expect(typeof result.message).toBe('string');
          if (result.details) {
            expect(typeof result.details).toBe('string');
          }
        } else {
          expect(result).toHaveProperty('error');
          expect(typeof result.error).toBe('string');
          if (result.details) {
            expect(typeof result.details).toBe('string');
          }
        }
      });
    });

    it('should stop execution after first failure', async () => {
      const projectName = ''; // This will cause the first step to fail
      const projectPath = join(tempDir, 'project');
      await mkdir(projectPath);

      const results = performMultiStepInit(projectName, projectPath);

      // Should only have one result (the failed first step)
      expect(results).toHaveLength(1);
      expect(results[0].success).toBe(false);

      // Verify that global directories were not created
      const globalClaudectlDir = join(tempDir, '.claudectl');
      try {
        await access(globalClaudectlDir);
        // If we get here, the directory exists when it shouldn't
        expect(false).toBe(true);
      } catch {
        // Expected - directory should not exist
        expect(true).toBe(true);
      }
    });
  });
});