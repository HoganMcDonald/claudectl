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
  success,
  instruction,
  section,
  table,
  blank,
  dim,
  emphasis,
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
 * Formats a worktree path for display.
 *
 * @param worktreePath - The full worktree path.
 * @param projectName - The project name.
 * @returns A user-friendly display path.
 */
function formatWorktreePath(worktreePath: string, projectName: string): string {
  const name = getWorktreeName(worktreePath, projectName);
  if (name) {
    return `~/.claudectl/projects/${projectName}/${name}`;
  }
  
  // For main repository, show relative to home
  const homeDir = process.env.HOME || process.env.USERPROFILE || "";
  if (homeDir && worktreePath.startsWith(homeDir)) {
    return `~${worktreePath.substring(homeDir.length)}`;
  }
  
  return worktreePath;
}

/**
 * Formats the status column for a worktree.
 *
 * @param worktree - The worktree information.
 * @returns Formatted status string.
 */
function formatStatus(worktree: WorktreeInfo): string {
  const parts: string[] = [];
  
  if (worktree.isMain) {
    parts.push("main");
  }
  
  if (worktree.isCurrent) {
    parts.push("current");
  }
  
  if (worktree.isClean === true) {
    parts.push("clean");
  } else if (worktree.isClean === false) {
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
 * Gets the display name for a worktree.
 *
 * @param worktree - The worktree information.
 * @param projectName - The project name.
 * @returns The display name.
 */
function getWorktreeDisplayName(worktree: WorktreeInfo, projectName: string): string {
  if (worktree.isMain) {
    return "main";
  }
  
  const name = getWorktreeName(worktree.path, projectName);
  return name || path.basename(worktree.path);
}

/**
 * Lists all active worktrees for the current claudectl project.
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

  // Get all worktrees for this project
  let worktrees: WorktreeInfo[];
  try {
    worktrees = getProjectWorktrees(projectConfig.name, currentDir);
  } catch (err) {
    fatal(`failed to list worktrees: ${err instanceof Error ? err.message : String(err)}`);
  }

  // Display results
  section(`Worktrees for project "${projectConfig.name}"`);
  blank();

  if (worktrees.length === 0) {
    info("No worktrees found for this project");
    blank();
    info("Create a new worktree with: claudectl new [name]");
    return;
  }

  // Prepare table data
  const headers = ["Name", "Branch", "Commit", "Status", "Last Commit"];
  const rows = worktrees.map(worktree => [
    getWorktreeDisplayName(worktree, projectConfig.name),
    worktree.branch || "-",
    formatCommitHash(worktree.commit),
    formatStatus(worktree),
    formatCommitMessage(worktree.commitMessage)
  ]);

  // Display table
  table(headers, rows);
  blank();

  // Show helpful information
  const currentWorktree = worktrees.find(wt => wt.isCurrent);
  if (currentWorktree) {
    const currentName = getWorktreeDisplayName(currentWorktree, projectConfig.name);
    success(`Currently in worktree: ${currentName}`);
  }

  const otherWorktrees = worktrees.filter(wt => !wt.isCurrent);
  if (otherWorktrees.length > 0) {
    blank();
    emphasis("Switch to a worktree:");
    otherWorktrees.forEach(worktree => {
      const name = getWorktreeDisplayName(worktree, projectConfig.name);
      const displayPath = formatWorktreePath(worktree.path, projectConfig.name);
      dim(`  cd ${displayPath}  # ${name}`);
    });
  }

  blank();
  info("Create a new worktree with: claudectl new [name]");
};