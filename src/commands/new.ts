import * as path from "node:path";
import {
  isGitRepository,
  hasClaudectlConfig,
  loadProjectConfig,
  getProjectDir,
  createWorktree,
  getDefaultBranch,
} from "../utils";
import {
  error,
  info,
  success,
  indentedSuccess,
  instruction,
  step,
  blank,
  section,
  fatal,
} from "../output";

/**
 * Generates a unique worktree name with timestamp.
 *
 * @param baseName - Base name for the worktree (defaults to "context").
 * @returns A unique worktree name.
 */
function generateWorktreeName(baseName: string = "context"): string {
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
  return `${baseName}-${timestamp}`;
}

/**
 * Creates a new worktree for the current claudectl project.
 *
 * @param worktreeName - Optional name for the new worktree. If not provided, generates a unique name.
 */
export const newCommand = (worktreeName?: string): void => {
  const currentDir = process.cwd();

  // Check if current directory is a git repository
  if (!isGitRepository(currentDir)) {
    error("current directory is not a git repository");
    instruction(
      "ClaudeCtl requires a git repository. Please initialize one first:",
      ["git init", "git add .", 'git commit -m "Initial commit"']
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

  // Generate worktree name if not provided
  const resolvedWorktreeName = worktreeName || generateWorktreeName();
  
  // Get project directory path
  const projectDir = getProjectDir(projectConfig.name);
  const worktreePath = path.join(projectDir, resolvedWorktreeName);

  // Get default branch for messaging
  let defaultBranch: string;
  try {
    defaultBranch = getDefaultBranch(currentDir);
  } catch (err) {
    fatal(`failed to determine default branch: ${err instanceof Error ? err.message : String(err)}`);
  }

  // Create the worktree
  section(`Creating new worktree "${resolvedWorktreeName}"`);
  blank();

  try {
    step(1, 2, `Fetching latest ${defaultBranch} from origin`);
    step(2, 2, `Creating worktree with branch ${resolvedWorktreeName}`);
    
    createWorktree(worktreePath, resolvedWorktreeName, defaultBranch, currentDir);
    
    indentedSuccess(`Worktree created at ${worktreePath}`);
    indentedSuccess(`Branch "${resolvedWorktreeName}" created from origin/${defaultBranch}`);
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : String(err);
    fatal(`failed to create worktree: ${errorMessage}`);
  }

  blank();
  success(`Worktree "${resolvedWorktreeName}" created successfully`);
  info(`Switch to the worktree: cd ${worktreePath}`);
  info("The worktree contains a fresh branch based on the latest main/master");
};