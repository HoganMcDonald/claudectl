import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { mkdtemp, rm, mkdir, access } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import {
  getGlobalClaudectlDir,
  getProjectsDir,
  getProjectDir,
  ensureDirectory,
  hasClaudectlConfig,
} from './directories';

describe('directory utilities', () => {
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

  describe('getGlobalClaudectlDir', () => {
    it('should return the correct global directory path', () => {
      const result = getGlobalClaudectlDir();
      expect(result).toBe(join(tempDir, '.claudectl'));
    });

    it('should throw error when home directory cannot be determined', () => {
      const originalHome = process.env.HOME;
      const originalUserProfile = process.env.USERPROFILE;
      
      delete process.env.HOME;
      delete process.env.USERPROFILE;
      
      expect(() => getGlobalClaudectlDir()).toThrow('Unable to determine home directory');
      
      // Restore environment variables
      if (originalHome) process.env.HOME = originalHome;
      if (originalUserProfile) process.env.USERPROFILE = originalUserProfile;
    });

    it('should use USERPROFILE on Windows when HOME is not available', () => {
      const originalHome = process.env.HOME;
      delete process.env.HOME;
      process.env.USERPROFILE = tempDir;
      
      const result = getGlobalClaudectlDir();
      expect(result).toBe(join(tempDir, '.claudectl'));
      
      // Restore HOME
      if (originalHome) process.env.HOME = originalHome;
    });
  });

  describe('getProjectsDir', () => {
    it('should return the correct projects directory path', () => {
      const result = getProjectsDir();
      expect(result).toBe(join(tempDir, '.claudectl', 'projects'));
    });
  });

  describe('getProjectDir', () => {
    it('should return the correct project-specific directory path', () => {
      const projectName = 'test-project';
      const result = getProjectDir(projectName);
      expect(result).toBe(join(tempDir, '.claudectl', 'projects', projectName));
    });

    it('should handle project names with special characters', () => {
      const projectName = 'test-project-with-dashes_and_underscores';
      const result = getProjectDir(projectName);
      expect(result).toBe(join(tempDir, '.claudectl', 'projects', projectName));
    });
  });

  describe('ensureDirectory', () => {
    it('should create a directory if it does not exist', async () => {
      const dirPath = join(tempDir, 'new-directory');
      
      ensureDirectory(dirPath);
      
      // Check that directory was created
      await expect(access(dirPath)).resolves.toBeUndefined();
    });

    it('should create nested directories recursively', async () => {
      const dirPath = join(tempDir, 'level1', 'level2', 'level3');
      
      ensureDirectory(dirPath);
      
      // Check that nested directory was created
      await expect(access(dirPath)).resolves.toBeUndefined();
    });

    it('should not throw if directory already exists', async () => {
      const dirPath = join(tempDir, 'existing-directory');
      await mkdir(dirPath);
      
      expect(() => ensureDirectory(dirPath)).not.toThrow();
    });

    it('should throw error if directory creation fails', () => {
      // Try to create a directory in a non-existent parent that we can't create
      // On most systems, this would be something like /root/... but we'll simulate
      const invalidPath = '/invalid/path/that/cannot/be/created';
      
      expect(() => ensureDirectory(invalidPath)).toThrow();
    });
  });

  describe('hasClaudectlConfig', () => {
    it('should return true if .claudectl directory exists', async () => {
      const projectDir = join(tempDir, 'test-project');
      await mkdir(projectDir);
      await mkdir(join(projectDir, '.claudectl'));
      
      const result = hasClaudectlConfig(projectDir);
      expect(result).toBe(true);
    });

    it('should return false if .claudectl directory does not exist', async () => {
      const projectDir = join(tempDir, 'test-project');
      await mkdir(projectDir);
      
      const result = hasClaudectlConfig(projectDir);
      expect(result).toBe(false);
    });

    it('should use current working directory when no path provided', () => {
      // Mock process.cwd to return our temp directory
      const originalCwd = process.cwd;
      vi.spyOn(process, 'cwd').mockReturnValue(tempDir);
      
      const result = hasClaudectlConfig();
      expect(result).toBe(false);
      
      // Restore original cwd
      process.cwd = originalCwd;
    });
  });
});