import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { mkdtemp, rm } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { rmCommand } from './rm';
import type { WorktreeInfo } from '../utils';

// Mock all utilities and output functions
vi.mock('../utils', () => ({
  isGitRepository: vi.fn(),
  hasClaudectlConfig: vi.fn(),
  loadProjectConfig: vi.fn(),
  findWorktreeByName: vi.fn(),
  removeWorktreeByName: vi.fn(),
  getWorktreeName: vi.fn(),
}));

vi.mock('../output', () => ({
  error: vi.fn(),
  info: vi.fn(),
  success: vi.fn(),
  warning: vi.fn(),
  instruction: vi.fn(),
  section: vi.fn(),
  blank: vi.fn(),
  emphasis: vi.fn(),
  fatal: vi.fn(),
}));

// Import the mocked functions
const { isGitRepository, hasClaudectlConfig, loadProjectConfig, findWorktreeByName, removeWorktreeByName, getWorktreeName } = await import('../utils');
const { error, info, success, warning, instruction, section, emphasis, fatal } = await import('../output');

describe('rm command', () => {
  let tempDir: string;
  let originalCwd: string;
  let processExitSpy: ReturnType<typeof vi.spyOn>;

  beforeEach(async () => {
    tempDir = await mkdtemp(join(tmpdir(), 'claudectl-test-'));
    originalCwd = process.cwd();
    
    // Mock process.cwd to return our temp directory
    vi.spyOn(process, 'cwd').mockReturnValue(tempDir);
    
    // Mock process.exit to throw instead of actually exiting
    processExitSpy = vi.spyOn(process, 'exit').mockImplementation(() => {
      throw new Error('process.exit() was called');
    });

    // Mock fatal to throw instead of calling process.exit
    vi.mocked(fatal).mockImplementation((message: string) => {
      throw new Error(`fatal: ${message}`);
    });

    // Reset all mocks
    vi.clearAllMocks();
  });

  afterEach(async () => {
    if (tempDir) {
      await rm(tempDir, { recursive: true, force: true });
    }
    vi.restoreAllMocks();
    process.chdir(originalCwd);
  });

  describe('successful removal', () => {
    it('should remove a clean worktree', () => {
      // Setup: git repo, claudectl config exists, clean worktree to remove
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const mockWorktree: WorktreeInfo = {
        path: '/home/.claudectl/projects/test-project/brave-penguin',
        branch: 'brave-penguin',
        commit: 'abc1234567890',
        isMain: false,
        isCurrent: false,
        isClean: true,
        commitMessage: 'Add user authentication'
      };
      
      vi.mocked(findWorktreeByName).mockReturnValue(mockWorktree);
      vi.mocked(getWorktreeName).mockReturnValue('brave-penguin');
      vi.mocked(removeWorktreeByName).mockReturnValue(mockWorktree);

      rmCommand('brave-penguin');

      // Should show section header
      expect(section).toHaveBeenCalledWith('Removing session "brave-penguin"');

      // Should show worktree information
      expect(info).toHaveBeenCalledWith('Session: brave-penguin');
      expect(info).toHaveBeenCalledWith('Branch: brave-penguin');
      expect(info).toHaveBeenCalledWith('Path: ~/.claudectl/projects/test-project/brave-penguin');
      expect(info).toHaveBeenCalledWith('Last commit: Add user authentication');

      // Should remove the worktree
      expect(removeWorktreeByName).toHaveBeenCalledWith('brave-penguin', 'test-project', tempDir, false);

      // Should show success message
      expect(success).toHaveBeenCalledWith('Session "brave-penguin" removed successfully');
      expect(info).toHaveBeenCalledWith('Removed directory: ~/.claudectl/projects/test-project/brave-penguin');
      expect(info).toHaveBeenCalledWith('Removed branch: brave-penguin');
    });

    it('should remove a dirty worktree with --force flag', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const mockWorktree: WorktreeInfo = {
        path: '/home/.claudectl/projects/test-project/swift-fox',
        branch: 'swift-fox',
        commit: 'def2345678901',
        isMain: false,
        isCurrent: false,
        isClean: false, // Dirty worktree
        commitMessage: 'Work in progress'
      };
      
      vi.mocked(findWorktreeByName).mockReturnValue(mockWorktree);
      vi.mocked(getWorktreeName).mockReturnValue('swift-fox');
      vi.mocked(removeWorktreeByName).mockReturnValue(mockWorktree);

      rmCommand('swift-fox', { force: true });

      // Should show warning about uncommitted changes
      expect(warning).toHaveBeenCalledWith('Forcing removal of session with uncommitted changes');

      // Should remove the worktree with force flag
      expect(removeWorktreeByName).toHaveBeenCalledWith('swift-fox', 'test-project', tempDir, true);

      // Should show success message
      expect(success).toHaveBeenCalledWith('Session "swift-fox" removed successfully');
    });

    it('should handle worktree without commit message', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const mockWorktree: WorktreeInfo = {
        path: '/home/.claudectl/projects/test-project/test-session',
        branch: 'test-session',
        commit: 'abc1234567890',
        isMain: false,
        isCurrent: false,
        isClean: true,
        commitMessage: undefined // No commit message
      };
      
      vi.mocked(findWorktreeByName).mockReturnValue(mockWorktree);
      vi.mocked(getWorktreeName).mockReturnValue('test-session');
      vi.mocked(removeWorktreeByName).mockReturnValue(mockWorktree);

      rmCommand('test-session');

      // Should not show commit message info
      expect(info).not.toHaveBeenCalledWith(expect.stringContaining('Last commit:'));
      
      // Should still remove successfully
      expect(success).toHaveBeenCalledWith('Session "test-session" removed successfully');
    });
  });

  describe('error conditions', () => {
    it('should exit if no session name provided', () => {
      expect(() => rmCommand('')).toThrow('process.exit() was called');

      expect(error).toHaveBeenCalledWith('session name is required');
      expect(instruction).toHaveBeenCalledWith(
        'Specify the name of the session to remove:',
        ['claudectl rm brave-penguin', 'claudectl rm swift-fox']
      );
      expect(processExitSpy).toHaveBeenCalledWith(1);
    });

    it('should exit if session name is whitespace only', () => {
      expect(() => rmCommand('   ')).toThrow('process.exit() was called');

      expect(error).toHaveBeenCalledWith('session name is required');
      expect(processExitSpy).toHaveBeenCalledWith(1);
    });

    it('should exit if current directory is not a git repository', () => {
      vi.mocked(isGitRepository).mockReturnValue(false);

      expect(() => rmCommand('test-session')).toThrow('process.exit() was called');

      expect(error).toHaveBeenCalledWith('current directory is not a git repository');
      expect(instruction).toHaveBeenCalledWith(
        'ClaudeCtl requires a git repository. Please navigate to one:',
        ['cd /path/to/your/git/project', 'claudectl rm test-session']
      );
      expect(processExitSpy).toHaveBeenCalledWith(1);
    });

    it('should exit if current directory is not a claudectl project', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);

      expect(() => rmCommand('test-session')).toThrow('process.exit() was called');

      expect(error).toHaveBeenCalledWith('current directory is not a claudectl project');
      expect(instruction).toHaveBeenCalledWith(
        'Please initialize a claudectl project first:',
        ['claudectl init']
      );
      expect(processExitSpy).toHaveBeenCalledWith(1);
    });

    it('should handle project config loading failure', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockImplementation(() => {
        throw new Error('Config not found');
      });

      expect(() => rmCommand('test-session')).toThrow('fatal: failed to load project configuration');

      expect(fatal).toHaveBeenCalledWith('failed to load project configuration');
    });

    it('should exit if session not found', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      vi.mocked(findWorktreeByName).mockReturnValue(null);

      expect(() => rmCommand('nonexistent-session')).toThrow('process.exit() was called');

      expect(error).toHaveBeenCalledWith('session "nonexistent-session" not found');
      expect(info).toHaveBeenCalledWith('List available sessions with: claudectl list');
      expect(processExitSpy).toHaveBeenCalledWith(1);
    });

    it('should exit if session has uncommitted changes without --force', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const mockWorktree: WorktreeInfo = {
        path: '/home/.claudectl/projects/test-project/dirty-session',
        branch: 'dirty-session',
        commit: 'abc1234567890',
        isMain: false,
        isCurrent: false,
        isClean: false, // Has uncommitted changes
        commitMessage: 'Work in progress'
      };
      
      vi.mocked(findWorktreeByName).mockReturnValue(mockWorktree);
      vi.mocked(getWorktreeName).mockReturnValue('dirty-session');

      expect(() => rmCommand('dirty-session')).toThrow('process.exit() was called');

      expect(warning).toHaveBeenCalledWith('Session has uncommitted changes!');
      expect(emphasis).toHaveBeenCalledWith('The following work will be lost:');
      expect(instruction).toHaveBeenCalledWith(
        'To remove anyway, use the --force flag:',
        ['claudectl rm dirty-session --force']
      );
      expect(processExitSpy).toHaveBeenCalledWith(1);
    });

    it('should exit if trying to remove main repository', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const mockWorktree: WorktreeInfo = {
        path: '/home/user/test-project',
        branch: 'main',
        commit: 'abc1234567890',
        isMain: true, // Main repository
        isCurrent: false,
        isClean: true,
        commitMessage: 'Initial commit'
      };
      
      vi.mocked(findWorktreeByName).mockReturnValue(mockWorktree);

      expect(() => rmCommand('main')).toThrow('process.exit() was called');

      expect(error).toHaveBeenCalledWith('cannot remove main repository');
      expect(info).toHaveBeenCalledWith('The main repository cannot be removed as it contains the primary codebase');
      expect(processExitSpy).toHaveBeenCalledWith(1);
    });

    it('should exit if trying to remove current worktree', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const mockWorktree: WorktreeInfo = {
        path: '/home/.claudectl/projects/test-project/current-session',
        branch: 'current-session',
        commit: 'abc1234567890',
        isMain: false,
        isCurrent: true, // Current worktree
        isClean: true,
        commitMessage: 'Current work'
      };
      
      vi.mocked(findWorktreeByName).mockReturnValue(mockWorktree);

      expect(() => rmCommand('current-session')).toThrow('process.exit() was called');

      expect(error).toHaveBeenCalledWith('cannot remove current session');
      expect(instruction).toHaveBeenCalledWith(
        'Switch to another session first, then remove this one:',
        [
          'cd ~/your-project  # switch to main',
          'claudectl rm current-session  # then remove'
        ]
      );
      expect(processExitSpy).toHaveBeenCalledWith(1);
    });

    it('should handle worktree removal failure', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const mockWorktree: WorktreeInfo = {
        path: '/home/.claudectl/projects/test-project/test-session',
        branch: 'test-session',
        commit: 'abc1234567890',
        isMain: false,
        isCurrent: false,
        isClean: true,
        commitMessage: 'Test work'
      };
      
      vi.mocked(findWorktreeByName).mockReturnValue(mockWorktree);
      vi.mocked(getWorktreeName).mockReturnValue('test-session');
      vi.mocked(removeWorktreeByName).mockImplementation(() => {
        throw new Error('Git command failed');
      });

      expect(() => rmCommand('test-session')).toThrow('fatal: failed to remove session: Git command failed');

      expect(fatal).toHaveBeenCalledWith('failed to remove session: Git command failed');
    });
  });

  describe('integration with utilities', () => {
    it('should pass correct parameters to utility functions', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'my-project' });
      
      const mockWorktree: WorktreeInfo = {
        path: '/home/.claudectl/projects/my-project/test-session',
        branch: 'test-session',
        commit: 'abc1234567890',
        isMain: false,
        isCurrent: false,
        isClean: true,
        commitMessage: 'Test'
      };
      
      vi.mocked(findWorktreeByName).mockReturnValue(mockWorktree);
      vi.mocked(getWorktreeName).mockReturnValue('test-session');
      vi.mocked(removeWorktreeByName).mockReturnValue(mockWorktree);

      rmCommand('test-session', { force: true });

      expect(isGitRepository).toHaveBeenCalledWith(tempDir);
      expect(hasClaudectlConfig).toHaveBeenCalledWith(tempDir);
      expect(loadProjectConfig).toHaveBeenCalledWith(tempDir);
      expect(findWorktreeByName).toHaveBeenCalledWith('test-session', 'my-project', tempDir);
      expect(removeWorktreeByName).toHaveBeenCalledWith('test-session', 'my-project', tempDir, true);
    });
  });
});