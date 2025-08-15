import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { mkdtemp, rm } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { initCommand } from './init';

// Mock all utilities and output functions
vi.mock('../utils', () => ({
  isGitRepository: vi.fn(),
  hasClaudectlConfig: vi.fn(),
  performMultiStepInit: vi.fn(),
}));

vi.mock('../output', () => ({
  error: vi.fn(),
  info: vi.fn(),
  success: vi.fn(),
  indentedSuccess: vi.fn(),
  indentedError: vi.fn(),
  instruction: vi.fn(),
  step: vi.fn(),
  blank: vi.fn(),
  section: vi.fn(),
}));

// Import the mocked functions
const { isGitRepository, hasClaudectlConfig, performMultiStepInit } = await import('../utils');
const { error, info, success, indentedSuccess, indentedError, instruction, step, blank, section } = await import('../output');

describe('init command', () => {
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

  describe('successful initialization', () => {
    it('should initialize project with auto-detected name', () => {
      // Setup: git repo, no existing config, successful init
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: true, message: 'Project configuration', details: '.claudectl/config.json' },
        { success: true, message: 'Global directory structure', details: '~/.claudectl/projects/test-project' },
      ]);

      initCommand();

      // Should detect project name from directory
      const expectedProjectName = tempDir.split('/').pop() || '';
      expect(performMultiStepInit).toHaveBeenCalledWith(expectedProjectName, tempDir);

      // Should show section header
      expect(section).toHaveBeenCalledWith(`Initializing ClaudeCtl project "${expectedProjectName}"`);

      // Should show steps
      expect(step).toHaveBeenCalledWith(1, 2, 'Project configuration');
      expect(step).toHaveBeenCalledWith(2, 2, 'Global directory structure');

      // Should show indented success messages
      expect(indentedSuccess).toHaveBeenCalledWith('.claudectl/config.json');
      expect(indentedSuccess).toHaveBeenCalledWith('~/.claudectl/projects/test-project');

      // Should show final success message
      expect(success).toHaveBeenCalledWith(`ClaudeCtl project "${expectedProjectName}" initialized successfully`);
      expect(info).toHaveBeenCalledWith('You can now use ClaudeCtl commands in this project');
    });

    it('should initialize project with provided name', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: true, message: 'Project configuration', details: '.claudectl/config.json' },
        { success: true, message: 'Global directory structure', details: '~/.claudectl/projects/my-custom-project' },
      ]);

      initCommand('my-custom-project');

      expect(performMultiStepInit).toHaveBeenCalledWith('my-custom-project', tempDir);
      expect(section).toHaveBeenCalledWith('Initializing ClaudeCtl project "my-custom-project"');
      expect(success).toHaveBeenCalledWith('ClaudeCtl project "my-custom-project" initialized successfully');
    });
  });

  describe('error conditions', () => {
    it('should exit if current directory is not a git repository', () => {
      vi.mocked(isGitRepository).mockReturnValue(false);

      expect(() => initCommand()).toThrow('process.exit() was called');

      expect(error).toHaveBeenCalledWith('current directory is not a git repository');
      expect(instruction).toHaveBeenCalledWith(
        'ClaudeCtl uses git worktrees for managing code contexts. Please initialize a git repository first:',
        ['git init', 'git add .', 'git commit -m "Initial commit"']
      );
      expect(processExitSpy).toHaveBeenCalledWith(1);

      // Should not proceed with initialization
      expect(performMultiStepInit).not.toHaveBeenCalled();
    });

    it('should show info message if project already initialized', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);

      initCommand('existing-project');

      expect(info).toHaveBeenCalledWith('Project "existing-project" is already initialized');

      // Should not proceed with initialization
      expect(performMultiStepInit).not.toHaveBeenCalled();
      expect(processExitSpy).not.toHaveBeenCalled();
    });

    it('should handle partial initialization failure', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: true, message: 'Project configuration', details: '.claudectl/config.json' },
        { success: false, error: 'Failed to create global directory structure', details: 'Permission denied' },
      ]);

      expect(() => initCommand('test-project')).toThrow('process.exit() was called');

      // Should show successful step
      expect(step).toHaveBeenCalledWith(1, 2, 'Project configuration');
      expect(indentedSuccess).toHaveBeenCalledWith('.claudectl/config.json');

      // Should show failed step
      expect(step).toHaveBeenCalledWith(2, 2, 'Failed to create global directory structure');
      expect(indentedError).toHaveBeenCalledWith('Permission denied');

      // Should show partial completion error
      expect(error).toHaveBeenCalledWith('Initialization partially completed (1/2 steps successful)');
      expect(processExitSpy).toHaveBeenCalledWith(1);
    });

    it('should handle complete initialization failure', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: false, error: 'Failed to create project configuration', details: 'Invalid project name' },
      ]);

      expect(() => initCommand('')).toThrow('process.exit() was called');

      expect(step).toHaveBeenCalledWith(1, 1, 'Failed to create project configuration');
      expect(indentedError).toHaveBeenCalledWith('Invalid project name');
      expect(error).toHaveBeenCalledWith('Initialization partially completed (0/1 steps successful)');
      expect(processExitSpy).toHaveBeenCalledWith(1);
    });
  });

  describe('step display logic', () => {
    it('should handle steps without details', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: true, message: 'Project configuration' }, // No details
        { success: false, error: 'Some error' }, // No details
      ]);

      expect(() => initCommand()).toThrow('process.exit() was called');

      expect(step).toHaveBeenCalledWith(1, 2, 'Project configuration');
      expect(step).toHaveBeenCalledWith(2, 2, 'Some error');
      
      // Should not call indented functions when no details
      expect(indentedSuccess).not.toHaveBeenCalled();
      expect(indentedError).not.toHaveBeenCalled();
    });

    it('should always call blank() at appropriate times', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: true, message: 'Project configuration' },
      ]);

      initCommand();

      // Should call blank() after section header and after step processing
      expect(blank).toHaveBeenCalledTimes(2);
    });
  });

  describe('project name resolution', () => {
    it('should use provided project name over directory name', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: true, message: 'Project configuration' },
      ]);

      initCommand('custom-name');

      expect(performMultiStepInit).toHaveBeenCalledWith('custom-name', tempDir);
      expect(section).toHaveBeenCalledWith('Initializing ClaudeCtl project "custom-name"');
    });

    it('should handle edge cases in directory name detection', () => {
      // Mock a directory path that might have edge cases
      vi.spyOn(process, 'cwd').mockReturnValue('/some/complex-path.with.dots_and-dashes');
      
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: true, message: 'Project configuration' },
      ]);

      initCommand();

      expect(performMultiStepInit).toHaveBeenCalledWith('complex-path.with.dots_and-dashes', '/some/complex-path.with.dots_and-dashes');
    });
  });

  describe('integration with utilities', () => {
    it('should pass correct parameters to utility functions', () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([]);

      initCommand('test-project');

      expect(isGitRepository).toHaveBeenCalledWith(tempDir);
      expect(hasClaudectlConfig).toHaveBeenCalledWith(tempDir);
      expect(performMultiStepInit).toHaveBeenCalledWith('test-project', tempDir);
    });
  });
});