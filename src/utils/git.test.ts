import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { mkdtemp, rm, mkdir } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { execSync } from 'node:child_process';
import { isGitRepository } from './git';

describe('git utilities', () => {
  let tempDir: string;

  beforeEach(async () => {
    tempDir = await mkdtemp(join(tmpdir(), 'claudectl-test-'));
  });

  afterEach(async () => {
    if (tempDir) {
      await rm(tempDir, { recursive: true, force: true });
    }
  });

  describe('isGitRepository', () => {
    it('should return true for a directory with .git folder', async () => {
      // Create a .git directory
      await mkdir(join(tempDir, '.git'));
      
      const result = isGitRepository(tempDir);
      expect(result).toBe(true);
    });

    it('should return false for a directory without .git folder', () => {
      const result = isGitRepository(tempDir);
      expect(result).toBe(false);
    });

    it('should return true for a real git repository', async () => {
      // Initialize a real git repository
      execSync('git init', { cwd: tempDir, stdio: 'pipe' });
      
      const result = isGitRepository(tempDir);
      expect(result).toBe(true);
    });

    it('should return false for a non-existent directory', () => {
      const result = isGitRepository('/non/existent/path');
      expect(result).toBe(false);
    });

    it('should use current working directory when no path provided', () => {
      // This test assumes we're running from a git repository (which we are)
      const result = isGitRepository();
      expect(result).toBe(true);
    });

    it('should handle permission errors gracefully', () => {
      // Test with a path that would cause permission issues
      const result = isGitRepository('/root/.ssh');
      expect(typeof result).toBe('boolean');
    });
  });
});