import * as path from "node:path";
import {
  isGitRepository,
  hasClaudectlConfig,
  loadProjectConfig,
  getProjectWorktrees,
  getWorktreeName,
  type WorktreeInfo,
} from "../utils";
import {
  error,
  info,
  instruction,
  section,
  table,
  blank,
  fatal,
} from "../output";

/**
 * Formats a commit hash for display (short version).
 *
 * @param commit - The full commit hash.
 * @returns The short commit hash (first 7 characters).
 */
function formatCommitHash(commit: string): string {
  return commit.substring(0, 7);
}

/**
 * Formats the status column for a task.
 *
 * @param task - The task information.
 * @returns Formatted status string.
 */
function formatStatus(task: WorktreeInfo): string {
  const parts: string[] = [];

  if (task.isMain) {
    parts.push("main");
  }

  if (task.isCurrent) {
    parts.push("current");
  }

  if (task.isClean === true) {
    parts.push("clean");
  } else if (task.isClean === false) {
    parts.push("dirty");
  }

  return parts.join(", ") || "-";
}

/**
 * Formats commit message for display (truncated if too long).
 *
 * @param message - The commit message.
 * @param maxLength - Maximum length to display (defaults to 50).
 * @returns Formatted commit message.
 */
function formatCommitMessage(message: string | undefined, maxLength: number = 50): string {
  if (!message) {
    return "-";
  }

  if (message.length <= maxLength) {
    return message;
  }

  return `${message.substring(0, maxLength - 3)}...`;
}

/**
 * Gets the display name for a task.
 *
 * @param task - The task information.
 * @param projectName - The project name.
 * @returns The display name.
 */
function getTaskDisplayName(task: WorktreeInfo, projectName: string): string {
  if (task.isMain) {
    return "main";
  }

  const name = getWorktreeName(task.path, projectName);
  return name || path.basename(task.path);
}

/**
 * Lists all active tasks for the current claudectl project.
 */
export const listCommand = (): void => {
  const currentDir = process.cwd();

  // Check if current directory is a git repository
  if (!isGitRepository(currentDir)) {
    error("current directory is not a git repository");
    instruction(
      "ClaudeCtl requires a git repository. Please navigate to one:",
      ["cd /path/to/your/git/project", "claudectl list"]
    );
    process.exit(1);
  }

  // Check if project is initialized
  if (!hasClaudectlConfig(currentDir)) {
    error("current directory is not a claudectl project");
    instruction(
      "Please initialize a claudectl project first:",
      ["claudectl init"]
    );
    process.exit(1);
  }

  // Load project configuration
  let projectConfig: { name: string };
  try {
    projectConfig = loadProjectConfig(currentDir);
  } catch (_err) {
    fatal("failed to load project configuration");
  }

  // Get all tasks for this project
  let tasks: WorktreeInfo[];
  try {
    tasks = getProjectWorktrees(projectConfig.name, currentDir);
  } catch (err) {
    fatal(`failed to list tasks: ${err instanceof Error ? err.message : String(err)}`);
  }

  // Display results
  section(`Tasks for project "${projectConfig.name}"`);
  blank();

  if (tasks.length === 0) {
    info("No tasks found for this project");
    blank();
    info("Create a new task with: claudectl new [name]");
    return;
  }

  // Prepare table data
  const headers = ["Name", "Branch", "Commit", "Status", "Last Commit"];
  const rows = tasks.map(task => [
    getTaskDisplayName(task, projectConfig.name),
    task.branch || "-",
    formatCommitHash(task.commit),
    formatStatus(task),
    formatCommitMessage(task.commitMessage)
  ]);

  // Display table
  table(headers, rows);
};
