import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { mkdtemp, rm } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { listCommand } from './list';
import type { WorktreeInfo } from '../utils';

// Mock all utilities and output functions
vi.mock('../utils', () => ({
  isGitRepository: vi.fn(),
  hasClaudectlConfig: vi.fn(),
  loadProjectConfig: vi.fn(),
  getProjectWorktrees: vi.fn(),
  getWorktreeName: vi.fn(),
}));

vi.mock('../output', () => ({
  error: vi.fn(),
  info: vi.fn(),
  success: vi.fn(),
  instruction: vi.fn(),
  section: vi.fn(),
  table: vi.fn(),
  blank: vi.fn(),
  dim: vi.fn(),
  emphasis: vi.fn(),
  fatal: vi.fn(),
}));

// Import the mocked functions
const { isGitRepository, hasClaudectlConfig, loadProjectConfig, getProjectWorktrees, getWorktreeName } = await import('../utils');
const { error, info, success, instruction, section, table, dim, emphasis, fatal } = await import('../output');

describe('list command', () => {
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

  describe('successful listing', () => {
    it('should list worktrees in table format', () => {
      // Setup: git repo, claudectl config exists, worktrees available
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const mockWorktrees: WorktreeInfo[] = [
        {
          path: '/home/user/test-project',
          branch: 'main',
          commit: 'abc1234567890',
          isMain: true,
          isCurrent: false,
          isClean: true,
          commitMessage: 'Initial commit'
        },
        {
          path: '/home/.claudectl/projects/test-project/brave-penguin',
          branch: 'brave-penguin',
          commit: 'def2345678901',
          isMain: false,
          isCurrent: true,
          isClean: false,
          commitMessage: 'Add user authentication feature'
        }
      ];
      
      vi.mocked(getProjectWorktrees).mockReturnValue(mockWorktrees);
      vi.mocked(getWorktreeName).mockImplementation((path, _projectName) => {
        if (path.includes('brave-penguin')) return 'brave-penguin';
        return null;
      });

      listCommand();

      // Should show section header
      expect(section).toHaveBeenCalledWith('Worktrees for project "test-project"');

      // Should display table with correct headers
      expect(table).toHaveBeenCalledWith(
        ['Name', 'Branch', 'Commit', 'Status', 'Last Commit'],
        [
          ['main', 'main', 'abc1234', 'main, clean', 'Initial commit'],
          ['brave-penguin', 'brave-penguin', 'def2345', 'current, dirty', 'Add user authentication feature']
        ]
      );

      // Should show current worktree info
      expect(success).toHaveBeenCalledWith('Currently in worktree: brave-penguin');
      
      // Should show switch instructions
      expect(emphasis).toHaveBeenCalledWith('Switch to a worktree:');
      expect(dim).toHaveBeenCalledWith('  cd /home/user/test-project  # main');
    });

    it('should handle project with no worktrees', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'empty-project' });
      vi.mocked(getProjectWorktrees).mockReturnValue([]);

      listCommand();

      expect(section).toHaveBeenCalledWith('Worktrees for project "empty-project"');
      expect(info).toHaveBeenCalledWith('No worktrees found for this project');
      expect(info).toHaveBeenCalledWith('Create a new worktree with: claudectl new [name]');
      
      // Should not display table
      expect(table).not.toHaveBeenCalled();
    });

    it('should handle worktree with missing information', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const mockWorktrees: WorktreeInfo[] = [
        {
          path: '/home/user/test-project',
          branch: 'main',
          commit: 'abc1234567890',
          isMain: true,
          isCurrent: true,
          isClean: undefined, // Unknown status
          commitMessage: undefined // No commit message
        }
      ];
      
      vi.mocked(getProjectWorktrees).mockReturnValue(mockWorktrees);
      vi.mocked(getWorktreeName).mockReturnValue(null);

      listCommand();

      expect(table).toHaveBeenCalledWith(
        ['Name', 'Branch', 'Commit', 'Status', 'Last Commit'],
        [
          ['main', 'main', 'abc1234', 'main, current', '-']
        ]
      );
    });

    it('should truncate long commit messages', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const longMessage = 'This is a very long commit message that should be truncated when displayed in the table';
      const mockWorktrees: WorktreeInfo[] = [
        {
          path: '/home/user/test-project',
          branch: 'main',
          commit: 'abc1234567890',
          isMain: true,
          isCurrent: true,
          isClean: true,
          commitMessage: longMessage
        }
      ];
      
      vi.mocked(getProjectWorktrees).mockReturnValue(mockWorktrees);
      vi.mocked(getWorktreeName).mockReturnValue(null);

      listCommand();

      expect(table).toHaveBeenCalledWith(
        ['Name', 'Branch', 'Commit', 'Status', 'Last Commit'],
        [
          ['main', 'main', 'abc1234', 'main, current, clean', 'This is a very long commit message that should ...']
        ]
      );
    });
  });

  describe('error conditions', () => {
    it('should exit if current directory is not a git repository', () => {
      vi.mocked(isGitRepository).mockReturnValue(false);

      expect(() => listCommand()).toThrow('process.exit() was called');

      expect(error).toHaveBeenCalledWith('current directory is not a git repository');
      expect(instruction).toHaveBeenCalledWith(
        'ClaudeCtl requires a git repository. Please navigate to one:',
        ['cd /path/to/your/git/project', 'claudectl list']
      );
      expect(processExitSpy).toHaveBeenCalledWith(1);

      // Should not proceed with listing
      expect(getProjectWorktrees).not.toHaveBeenCalled();
    });

    it('should exit if current directory is not a claudectl project', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);

      expect(() => listCommand()).toThrow('process.exit() was called');

      expect(error).toHaveBeenCalledWith('current directory is not a claudectl project');
      expect(instruction).toHaveBeenCalledWith(
        'Please initialize a claudectl project first:',
        ['claudectl init']
      );
      expect(processExitSpy).toHaveBeenCalledWith(1);

      // Should not proceed with listing
      expect(getProjectWorktrees).not.toHaveBeenCalled();
    });

    it('should handle project config loading failure', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockImplementation(() => {
        throw new Error('Config not found');
      });

      expect(() => listCommand()).toThrow('fatal: failed to load project configuration');

      expect(fatal).toHaveBeenCalledWith('failed to load project configuration');

      // Should not proceed with listing
      expect(getProjectWorktrees).not.toHaveBeenCalled();
    });

    it('should handle worktree listing failure', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      vi.mocked(getProjectWorktrees).mockImplementation(() => {
        throw new Error('Git command failed');
      });

      expect(() => listCommand()).toThrow('fatal: failed to list worktrees: Git command failed');

      expect(fatal).toHaveBeenCalledWith('failed to list worktrees: Git command failed');
    });
  });

  describe('display formatting', () => {
    it('should correctly format commit hashes', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const mockWorktrees: WorktreeInfo[] = [
        {
          path: '/home/user/test-project',
          branch: 'main',
          commit: 'abcdef1234567890abcdef1234567890abcdef12',
          isMain: true,
          isCurrent: true,
          isClean: true,
          commitMessage: 'Test commit'
        }
      ];
      
      vi.mocked(getProjectWorktrees).mockReturnValue(mockWorktrees);
      vi.mocked(getWorktreeName).mockReturnValue(null);

      listCommand();

      // Should truncate commit hash to 7 characters
      expect(table).toHaveBeenCalledWith(
        ['Name', 'Branch', 'Commit', 'Status', 'Last Commit'],
        [
          ['main', 'main', 'abcdef1', 'main, current, clean', 'Test commit']
        ]
      );
    });

    it('should show helpful creation instructions', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      vi.mocked(getProjectWorktrees).mockReturnValue([]);

      listCommand();

      expect(info).toHaveBeenCalledWith('Create a new worktree with: claudectl new [name]');
    });

    it('should handle worktrees without branches', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'test-project' });
      
      const mockWorktrees: WorktreeInfo[] = [
        {
          path: '/home/user/test-project',
          branch: '', // No branch
          commit: 'abc1234567890',
          isMain: false,
          isCurrent: true,
          isClean: true,
          commitMessage: 'Detached HEAD'
        }
      ];
      
      vi.mocked(getProjectWorktrees).mockReturnValue(mockWorktrees);
      vi.mocked(getWorktreeName).mockReturnValue(null);

      listCommand();

      expect(table).toHaveBeenCalledWith(
        ['Name', 'Branch', 'Commit', 'Status', 'Last Commit'],
        [
          ['test-project', '-', 'abc1234', 'current, clean', 'Detached HEAD']
        ]
      );
    });
  });

  describe('integration with utilities', () => {
    it('should pass correct parameters to utility functions', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: 'my-project' });
      vi.mocked(getProjectWorktrees).mockReturnValue([]);

      listCommand();

      expect(isGitRepository).toHaveBeenCalledWith(tempDir);
      expect(hasClaudectlConfig).toHaveBeenCalledWith(tempDir);
      expect(loadProjectConfig).toHaveBeenCalledWith(tempDir);
      expect(getProjectWorktrees).toHaveBeenCalledWith('my-project', tempDir);
    });
  });
});