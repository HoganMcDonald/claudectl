import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import {
  error,
  success,
  indentedSuccess,
  indentedError,
  info,
  warning,
  blank,
  codeBlock,
  instruction,
  section,
  progress,
  fatal,
  rule,
  step,
  bullet,
  table,
  progressBar,
  spinner,
  clearLine,
  badge,
  emphasis,
  dim,
} from './output';

describe('output utilities', () => {
  let consoleLogSpy: ReturnType<typeof vi.spyOn>;
  let consoleErrorSpy: ReturnType<typeof vi.spyOn>;
  let stdoutWriteSpy: ReturnType<typeof vi.spyOn>;
  let processExitSpy: ReturnType<typeof vi.spyOn>;

  beforeEach(() => {
    consoleLogSpy = vi.spyOn(console, 'log').mockImplementation(() => {});
    consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    stdoutWriteSpy = vi.spyOn(process.stdout, 'write').mockImplementation(() => true);
    processExitSpy = vi.spyOn(process, 'exit').mockImplementation(() => {
      throw new Error('process.exit() was called');
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('error', () => {
    it('should print error message with icon', () => {
      error('test error message');

      expect(consoleErrorSpy).toHaveBeenCalledOnce();
      const output = consoleErrorSpy.mock.calls[0][0];
      expect(output).toContain('✗');
      expect(output).toContain('error: test error message');
    });

    it('should print error message with details', () => {
      error('test error message', 'additional details');

      expect(consoleErrorSpy).toHaveBeenCalledTimes(2);
      expect(consoleErrorSpy.mock.calls[0][0]).toContain('test error message');
      expect(consoleErrorSpy.mock.calls[1][0]).toContain('additional details');
    });
  });

  describe('success', () => {
    it('should print success message with icon', () => {
      success('test success message');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('✓');
      expect(output).toContain('Created test success message');
    });
  });

  describe('indentedSuccess', () => {
    it('should print indented success message', () => {
      indentedSuccess('test message');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toMatch(/^\s{4}/); // Starts with 4 spaces
      expect(output).toContain('✓');
      expect(output).toContain('Created test message');
    });
  });

  describe('indentedError', () => {
    it('should print indented error message', () => {
      indentedError('test error');

      expect(consoleErrorSpy).toHaveBeenCalledOnce();
      const output = consoleErrorSpy.mock.calls[0][0];
      expect(output).toMatch(/^\s{4}/); // Starts with 4 spaces
      expect(output).toContain('✗');
      expect(output).toContain('test error');
    });
  });

  describe('info', () => {
    it('should print info message with icon', () => {
      info('test info message');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('ℹ');
      expect(output).toContain('test info message');
    });
  });

  describe('warning', () => {
    it('should print warning message with icon', () => {
      warning('test warning message');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('⚠');
      expect(output).toContain('warning: test warning message');
    });
  });

  describe('blank', () => {
    it('should print empty line', () => {
      blank();

      expect(consoleLogSpy).toHaveBeenCalledWith('');
    });
  });

  describe('codeBlock', () => {
    it('should print single line code block', () => {
      codeBlock('npm install');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('npm install');
      expect(output).toMatch(/^\s{4}/); // Indented with 4 spaces
    });

    it('should print multiple line code block', () => {
      codeBlock(['git init', 'git add .', 'git commit -m "Initial"']);

      expect(consoleLogSpy).toHaveBeenCalledTimes(3);
      expect(consoleLogSpy.mock.calls[0][0]).toContain('git init');
      expect(consoleLogSpy.mock.calls[1][0]).toContain('git add .');
      expect(consoleLogSpy.mock.calls[2][0]).toContain('git commit');
    });
  });

  describe('instruction', () => {
    it('should print instruction with title and commands', () => {
      instruction('Initialize git:', ['git init', 'git add .']);

      expect(consoleLogSpy).toHaveBeenCalledWith('Initialize git:');
      // Should include blank lines and code blocks
      expect(consoleLogSpy).toHaveBeenCalledWith('');
      const calls = consoleLogSpy.mock.calls;
      expect(calls.some(call => call[0].includes('git init'))).toBe(true);
      expect(calls.some(call => call[0].includes('git add .'))).toBe(true);
    });
  });

  describe('section', () => {
    it('should print section header', () => {
      section('Test Section');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('Test Section:');
    });
  });

  describe('progress', () => {
    it('should print progress message', () => {
      progress('Creating', 'project directory');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('Creating');
      expect(output).toContain('project directory');
      expect(output).toMatch(/^\s{5}/); // Indented with 5 spaces
    });
  });

  describe('fatal', () => {
    it('should print error and exit with default code', () => {
      expect(() => fatal('fatal error')).toThrow('process.exit() was called');
      expect(consoleErrorSpy).toHaveBeenCalled();
      expect(processExitSpy).toHaveBeenCalledWith(1);
    });

    it('should print error and exit with custom code', () => {
      expect(() => fatal('fatal error', 2)).toThrow('process.exit() was called');
      expect(processExitSpy).toHaveBeenCalledWith(2);
    });
  });

  describe('rule', () => {
    it('should print horizontal rule with default settings', () => {
      rule();

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('─'.repeat(50));
    });

    it('should print horizontal rule with custom length and character', () => {
      rule(20, '=');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('='.repeat(20));
    });
  });

  describe('step', () => {
    it('should print step indicator', () => {
      step(2, 5, 'Creating configuration');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('[2/5]');
      expect(output).toContain('→');
      expect(output).toContain('Creating configuration');
    });
  });

  describe('bullet', () => {
    it('should print bullet point at level 0', () => {
      bullet('Main item');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('•');
      expect(output).toContain('Main item');
      expect(output).not.toMatch(/^\s/); // No indentation at level 0
    });

    it('should print bullet point at level 1', () => {
      bullet('Sub item', 1);

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('•');
      expect(output).toContain('Sub item');
      expect(output).toMatch(/^\s{2}/); // 2 spaces indentation
    });

    it('should print bullet point at level 2', () => {
      bullet('Sub-sub item', 2);

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toMatch(/^\s{4}/); // 4 spaces indentation
    });
  });

  describe('table', () => {
    it('should print table with headers and rows', () => {
      table(['Name', 'Status'], [['project-1', 'active'], ['project-2', 'inactive']]);

      const calls = consoleLogSpy.mock.calls;
      expect(calls.length).toBeGreaterThan(2);
      
      // Check headers
      expect(calls[0][0]).toContain('Name');
      expect(calls[0][0]).toContain('Status');
      
      // Check separator line
      expect(calls[1][0]).toContain('─');
      
      // Check data rows
      expect(calls.some(call => call[0].includes('project-1'))).toBe(true);
      expect(calls.some(call => call[0].includes('project-2'))).toBe(true);
    });

    it('should handle empty rows', () => {
      table(['Name', 'Status'], []);

      expect(consoleLogSpy).not.toHaveBeenCalled();
    });
  });

  describe('progressBar', () => {
    it('should print progress bar with default settings', () => {
      progressBar(3, 10);

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('[');
      expect(output).toContain(']');
      expect(output).toContain('30%');
    });

    it('should print progress bar with label', () => {
      progressBar(7, 10, 20, 'Installing');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('Installing');
      expect(output).toContain('70%');
    });

    it('should handle progress values outside 0-1 range', () => {
      progressBar(-1, 10);
      expect(consoleLogSpy.mock.calls[0][0]).toContain('0%');

      progressBar(15, 10);
      expect(consoleLogSpy.mock.calls[1][0]).toContain('100%');
    });
  });

  describe('spinner', () => {
    it('should write spinner to stdout', () => {
      spinner('Loading...', 0);

      expect(stdoutWriteSpy).toHaveBeenCalledOnce();
      const output = stdoutWriteSpy.mock.calls[0][0];
      expect(output).toContain('Loading...');
      expect(output).toContain('\r'); // Carriage return for overwriting
    });

    it('should handle different frame indices', () => {
      spinner('Loading...', 5);
      const output = stdoutWriteSpy.mock.calls[0][0];
      expect(output).toContain('Loading...');
    });

    it('should cycle through frame indices', () => {
      spinner('Loading...', 15); // Should wrap around
      expect(stdoutWriteSpy).toHaveBeenCalled();
    });
  });

  describe('clearLine', () => {
    it('should write clear line sequence to stdout', () => {
      clearLine();

      expect(stdoutWriteSpy).toHaveBeenCalledWith('\r\x1b[K');
    });
  });

  describe('badge', () => {
    it('should print badge with default color', () => {
      badge('NEW');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('NEW');
    });

    it('should print badge with custom color', () => {
      badge('BETA', 'yellow');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('BETA');
    });
  });

  describe('emphasis', () => {
    it('should print emphasized text', () => {
      emphasis('Important message');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('Important message');
    });
  });

  describe('dim', () => {
    it('should print dimmed text', () => {
      dim('Secondary information');

      expect(consoleLogSpy).toHaveBeenCalledOnce();
      const output = consoleLogSpy.mock.calls[0][0];
      expect(output).toContain('Secondary information');
    });
  });

  describe('color support detection', () => {
    let originalEnv: NodeJS.ProcessEnv;
    let originalIsTTY: boolean;

    beforeEach(() => {
      originalEnv = { ...process.env };
      originalIsTTY = process.stdout.isTTY;
    });

    afterEach(() => {
      process.env = originalEnv;
      process.stdout.isTTY = originalIsTTY;
    });

    it('should handle environments without color support', () => {
      // Simulate environment without color support
      process.stdout.isTTY = false;
      delete process.env.COLORTERM;
      delete process.env.TERM;
      delete process.env.FORCE_COLOR;
      process.env.NO_COLOR = '1';

      success('test message');

      // Should still print the message, just without ANSI codes
      expect(consoleLogSpy).toHaveBeenCalled();
    });

    it('should handle environments with forced color support', () => {
      process.stdout.isTTY = false;
      process.env.FORCE_COLOR = '1';

      success('test message');

      expect(consoleLogSpy).toHaveBeenCalled();
    });
  });
});