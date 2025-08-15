/**
 * Consistent CLI output formatting utilities with colors and icons.
 *
 * This module provides standardized output functions to ensure consistent
 * formatting across the entire CLI application, following cargo-like conventions
 * with modern CLI enhancements like colors, icons, and progress indicators.
 */

// ANSI color codes for terminal output
const colors = {
  red: "\x1b[31m",
  green: "\x1b[32m",
  yellow: "\x1b[33m",
  blue: "\x1b[34m",
  magenta: "\x1b[35m",
  cyan: "\x1b[36m",
  white: "\x1b[37m",
  gray: "\x1b[90m",
  reset: "\x1b[0m",
  bold: "\x1b[1m",
  dim: "\x1b[2m",
} as const;

// Icons for different message types
const icons = {
  error: "✗",
  warning: "⚠",
  info: "ℹ",
  success: "✓",
  arrow: "→",
  bullet: "•",
  check: "✓",
  cross: "✗",
  spinner: ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
} as const;

/**
 * Checks if terminal supports colors by looking for common environment variables.
 */
function supportsColor(): boolean {
  return !!(
    process.stdout.isTTY &&
    (process.env.COLORTERM ||
      process.env.TERM === "xterm-256color" ||
      process.env.TERM === "xterm" ||
      process.env.FORCE_COLOR ||
      !process.env.NO_COLOR)
  );
}

/**
 * Applies color formatting if terminal supports it.
 */
function colorize(text: string, color: keyof typeof colors): string {
  return supportsColor() ? `${colors[color]}${text}${colors.reset}` : text;
}

/**
 * Prints an error message with red color and error icon.
 *
 * @param message - The error message to display.
 * @param details - Optional additional details to show below the main message.
 *
 * @example
 * ```typescript
 * error("current directory is not a git repository");
 * error("failed to initialize project", "Permission denied");
 * ```
 */
export function error(message: string, details?: string): void {
  const icon = colorize(icons.error, "red");
  const errorText = colorize(`error: ${message}`, "red");
  console.error(`${icon} ${errorText}`);
  if (details) {
    console.error(`   ${colorize(details, "dim")}`);
  }
}

/**
 * Prints a success message with green color and check icon.
 *
 * @param message - The success message to display.
 *
 * @example
 * ```typescript
 * success('ClaudeCtl project "my-project"');
 * success(".claudectl/config.json");
 * ```
 */
export function success(message: string): void {
  const icon = colorize(icons.success, "green");
  const text = colorize(`Created ${message}`, "green");
  console.log(`${icon} ${text}`);
}

/**
 * Prints an indented success message with green color and check icon.
 * Used for sub-items under step indicators.
 *
 * @param message - The success message to display.
 *
 * @example
 * ```typescript
 * indentedSuccess(".claudectl/config.json");
 * indentedSuccess("~/.claudectl/projects/my-project");
 * ```
 */
export function indentedSuccess(message: string): void {
  const icon = colorize(icons.success, "green");
  const text = colorize(`Created ${message}`, "green");
  console.log(`    ${icon} ${text}`);
}

/**
 * Prints an indented error message with red color and error icon.
 * Used for sub-items under step indicators.
 *
 * @param message - The error message to display.
 *
 * @example
 * ```typescript
 * indentedError("Permission denied");
 * ```
 */
export function indentedError(message: string): void {
  const icon = colorize(icons.error, "red");
  const text = colorize(message, "red");
  console.error(`    ${icon} ${text}`);
}

/**
 * Prints an informational message with blue color and info icon.
 *
 * @param message - The info message to display.
 *
 * @example
 * ```typescript
 * info('Project "my-project" is already initialized');
 * ```
 */
export function info(message: string): void {
  const icon = colorize(icons.info, "blue");
  const text = colorize(message, "blue");
  console.log(`${icon} ${text}`);
}

/**
 * Prints a warning message with yellow color and warning icon.
 *
 * @param message - The warning message to display.
 *
 * @example
 * ```typescript
 * warning("This operation will overwrite existing files");
 * ```
 */
export function warning(message: string): void {
  const icon = colorize(icons.warning, "yellow");
  const text = colorize(`warning: ${message}`, "yellow");
  console.log(`${icon} ${text}`);
}

/**
 * Prints a blank line for spacing.
 *
 * @example
 * ```typescript
 * error("Something went wrong");
 * blank();
 * info("Here's how to fix it:");
 * ```
 */
export function blank(): void {
  console.log("");
}

/**
 * Prints a code block with consistent indentation and dimmed styling.
 *
 * @param lines - Array of code lines to display, or a single string.
 *
 * @example
 * ```typescript
 * codeBlock([
 *   "git init",
 *   "git add .",
 *   'git commit -m "Initial commit"'
 * ]);
 *
 * codeBlock("npm install");
 * ```
 */
export function codeBlock(lines: string[] | string): void {
  const codeLines = Array.isArray(lines) ? lines : [lines];
  codeLines.forEach((line) => {
    const formattedLine = colorize(`    ${line}`, "dim");
    console.log(formattedLine);
  });
}

/**
 * Prints an instructional message block with title and code examples.
 *
 * @param title - The instructional title.
 * @param commands - Array of commands to show.
 *
 * @example
 * ```typescript
 * instruction(
 *   "Please initialize a git repository first:",
 *   ["git init", "git add .", 'git commit -m "Initial commit"']
 * );
 * ```
 */
export function instruction(title: string, commands: string[]): void {
  console.log(title);
  blank();
  codeBlock(commands);
  blank();
}

/**
 * Prints a section header for grouping related output.
 *
 * @param title - The section title.
 *
 * @example
 * ```typescript
 * section("Initializing project");
 * success("Created project directory");
 * success("Created configuration file");
 * ```
 */
export function section(title: string): void {
  console.log(colorize(`${title}:`, "bold"));
}

/**
 * Prints a progress-style message with action verb and colored formatting.
 *
 * @param action - The action being performed (e.g., "Creating", "Installing").
 * @param target - The target of the action.
 *
 * @example
 * ```typescript
 * progress("Creating", "project directory");
 * progress("Installing", "dependencies");
 * ```
 */
export function progress(action: string, target: string): void {
  const actionText = colorize(action, "cyan");
  const targetText = colorize(target, "dim");
  console.log(`     ${actionText} ${targetText}`);
}

/**
 * Exits the process with an error code after printing an error message.
 *
 * @param message - The error message to display before exiting.
 * @param code - The exit code (defaults to 1).
 *
 * @example
 * ```typescript
 * fatal("Current directory is not a git repository");
 * fatal("Permission denied", 2);
 * ```
 */
export function fatal(message: string, code: number = 1): never {
  error(message);
  process.exit(code);
}

// ========================================
// Additional Formatting Niceties
// ========================================

/**
 * Prints a horizontal rule/divider for section separation.
 *
 * @param length - Length of the rule (defaults to 50).
 * @param char - Character to use for the rule (defaults to '─').
 *
 * @example
 * ```typescript
 * rule();
 * rule(30, '=');
 * ```
 */
export function rule(length: number = 50, char: string = "─"): void {
  console.log(colorize(char.repeat(length), "dim"));
}

/**
 * Prints a step indicator with numbering for multi-step processes.
 *
 * @param step - Current step number.
 * @param total - Total number of steps.
 * @param message - Description of the current step.
 *
 * @example
 * ```typescript
 * step(1, 3, "Initializing project");
 * step(2, 3, "Creating configuration");
 * step(3, 3, "Setting up git repository");
 * ```
 */
export function step(step: number, total: number, message: string): void {
  const stepIndicator = colorize(`[${step}/${total}]`, "cyan");
  const arrow = colorize(icons.arrow, "cyan");
  console.log(`${stepIndicator} ${arrow} ${message}`);
}

/**
 * Prints a bullet point list item.
 *
 * @param message - The list item text.
 * @param level - Indentation level (0-based).
 *
 * @example
 * ```typescript
 * bullet("Main item");
 * bullet("Sub item", 1);
 * bullet("Sub-sub item", 2);
 * ```
 */
export function bullet(message: string, level: number = 0): void {
  const indent = "  ".repeat(level);
  const bulletIcon = colorize(icons.bullet, "gray");
  console.log(`${indent}${bulletIcon} ${message}`);
}

/**
 * Prints a table with aligned columns.
 *
 * @param headers - Array of column headers.
 * @param rows - Array of row data (each row is an array of strings).
 *
 * @example
 * ```typescript
 * table(['Name', 'Status', 'Path'], [
 *   ['project-1', 'active', '/path/to/project-1'],
 *   ['project-2', 'inactive', '/path/to/project-2']
 * ]);
 * ```
 */
export function table(headers: string[], rows: string[][]): void {
  if (rows.length === 0) return;

  // Calculate column widths
  const widths = headers.map((header, i) => {
    const maxRowWidth = Math.max(...rows.map((row) => (row[i] || "").length));
    return Math.max(header.length, maxRowWidth);
  });

  // Print headers
  const headerRow = headers
    .map((header, i) => colorize(header.padEnd(widths[i]), "bold"))
    .join("  ");
  console.log(headerRow);

  // Print separator
  const separator = widths.map((width) => "─".repeat(width)).join("  ");
  console.log(colorize(separator, "dim"));

  // Print rows
  rows.forEach((row) => {
    const formattedRow = row
      .map((cell, i) => (cell || "").padEnd(widths[i]))
      .join("  ");
    console.log(formattedRow);
  });
}

/**
 * Prints a simple progress bar.
 *
 * @param current - Current progress value.
 * @param total - Total/maximum progress value.
 * @param width - Width of the progress bar in characters (defaults to 20).
 * @param label - Optional label to display.
 *
 * @example
 * ```typescript
 * progressBar(3, 10, 20, "Installing dependencies");
 * progressBar(7, 10);
 * ```
 */
export function progressBar(
  current: number,
  total: number,
  width: number = 20,
  label?: string
): void {
  const percentage = Math.min(Math.max(current / total, 0), 1);
  const filled = Math.round(width * percentage);
  const empty = width - filled;

  const bar =
    colorize("█".repeat(filled), "green") + colorize("░".repeat(empty), "dim");
  const percent = `${Math.round(percentage * 100)}%`;

  const output = label ? `${label} [${bar}] ${percent}` : `[${bar}] ${percent}`;
  console.log(output);
}

/**
 * Prints a spinner for long-running operations (single frame).
 * Call this repeatedly with different frame indices for animation.
 *
 * @param message - Message to display with the spinner.
 * @param frameIndex - Current frame index (0-9).
 *
 * @example
 * ```typescript
 * // In a loop or interval:
 * spinner("Loading...", frameIndex % 10);
 * ```
 */
export function spinner(message: string, frameIndex: number = 0): void {
  const frame = icons.spinner[frameIndex % icons.spinner.length];
  const spinnerIcon = colorize(frame, "cyan");
  process.stdout.write(`\r${spinnerIcon} ${message}`);
}

/**
 * Clears the current line (useful after spinner animations).
 */
export function clearLine(): void {
  process.stdout.write("\r\x1b[K");
}

/**
 * Prints a badge/tag style label.
 *
 * @param label - The badge text.
 * @param color - Color for the badge background.
 *
 * @example
 * ```typescript
 * badge("NEW", "green");
 * badge("BETA", "yellow");
 * badge("DEPRECATED", "red");
 * ```
 */
export function badge(
  label: string,
  color: keyof typeof colors = "blue"
): void {
  const badgeText = colorize(` ${label} `, color);
  console.log(badgeText);
}

/**
 * Prints text with emphasis (bold).
 *
 * @param text - Text to emphasize.
 *
 * @example
 * ```typescript
 * emphasis("Important: This action cannot be undone");
 * ```
 */
export function emphasis(text: string): void {
  console.log(colorize(text, "bold"));
}

/**
 * Prints dimmed/muted text for less important information.
 *
 * @param text - Text to dim.
 *
 * @example
 * ```typescript
 * dim("This is supplementary information");
 * ```
 */
export function dim(text: string): void {
  console.log(colorize(text, "dim"));
}
