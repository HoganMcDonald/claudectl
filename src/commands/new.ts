import * as path from "node:path";
import {
  getProjectDir,
  createWorktree,
  getDefaultBranch,
  generateRandomName,
} from "../utils.js";
import {
  error,
  info,
  success,
  indentedSuccess,
  step,
  blank,
  section,
  fatal,
} from "../output.js";
import { ClaudeSessionManager } from "../claude-session.js";
import { handleProjectValidation } from "../utils/errors.js";


/**
 * Creates a new worktree for the current claudectl project and starts a Claude Code session.
 */
export const newCommand = async (worktreeName?: string): Promise<void> => {
  const currentDir = process.cwd();
  const projectConfig = handleProjectValidation(currentDir);

  // Generate worktree name if not provided
  const resolvedWorktreeName = worktreeName || generateRandomName();
  
  // Get project directory path
  const projectDir = getProjectDir(projectConfig.name);
  const worktreePath = path.join(projectDir, resolvedWorktreeName);

  // Get default branch for messaging
  let defaultBranch: string;
  try {
    defaultBranch = getDefaultBranch(currentDir);
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : String(err);
    fatal(`failed to determine default branch: ${errorMessage}`);
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

  // Start Claude Code session
  blank();
  section("Starting Claude Code session");
  blank();

  try {
    await ClaudeSessionManager.startSession({
      workingDirectory: worktreePath,
      sessionName: resolvedWorktreeName,
      useContainer: true,
      dangerouslySkipPermissions: true
    });
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : String(err);
    error(`Failed to start Claude Code session: ${errorMessage}`);
    info("You can manually start Claude Code later if needed");
  }

  blank();
  success(`Worktree "${resolvedWorktreeName}" created successfully`);
  success(`Claude Code session started in background`);
  info(`Switch to the worktree: cd ${worktreePath}`);
  info("The worktree contains a fresh branch based on the latest main/master");
  info("Claude Code is running with container isolation and dangerously-skip-permissions");
};