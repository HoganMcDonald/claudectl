import {
  findWorktreeByName,
  removeWorktreeByName,
  getWorktreeName,
  type WorktreeInfo,
} from "../utils.js";
import {
  error,
  info,
  success,
  warning,
  instruction,
  section,
  blank,
  emphasis,
  fatal,
} from "../output.js";
import { ClaudeSessionManager } from "../claude-session.js";
import { handleProjectValidation, validateSessionName } from "../utils/errors.js";

/**
 * Removes a worktree/session by name from the current claudectl project.
 * Also stops any associated Claude Code session.
 *
 * @param sessionName - Name of the session/worktree to remove.
 * @param options - Command options including force flag.
 */
/**
 * Removes a worktree/session by name from the current claudectl project.
 * Also stops any associated Claude Code session.
 */
export const rmCommand = async (sessionName: string, options: { force?: boolean } = {}): Promise<void> => {
  const currentDir = process.cwd();

  // Validate session name is provided
  validateSessionName(sessionName, "rm");
  const projectConfig = handleProjectValidation(currentDir, `rm ${sessionName}`);

  // Find the worktree
  let worktree: WorktreeInfo | null;
  try {
    worktree = findWorktreeByName(sessionName, projectConfig.name, currentDir);
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : String(err);
    fatal(`failed to find session: ${errorMessage}`);
  }

  if (!worktree) {
    error(`session "${sessionName}" not found`);
    blank();
    info("List available sessions with: claudectl list");
    process.exit(1);
  }

  // Show what will be removed
  section(`Removing session "${sessionName}"`);
  blank();

  const displayPath = getWorktreeName(worktree.path, projectConfig.name) 
    ? `~/.claudectl/projects/${projectConfig.name}/${sessionName}`
    : worktree.path;

  info(`Session: ${sessionName}`);
  info(`Branch: ${worktree.branch}`);
  info(`Path: ${displayPath}`);
  
  if (worktree.commitMessage) {
    info(`Last commit: ${worktree.commitMessage}`);
  }

  // Check for uncommitted changes
  if (worktree.isClean === false && !options.force) {
    blank();
    warning("Session has uncommitted changes!");
    emphasis("The following work will be lost:");
    info("• Uncommitted file changes");
    info("• Untracked files");
    blank();
    instruction(
      "To remove anyway, use the --force flag:",
      [`claudectl rm ${sessionName} --force`]
    );
    process.exit(1);
  }

  // Prevent removal of main repository
  if (worktree.isMain) {
    error('cannot remove main repository');
    blank();
    info("The main repository cannot be removed as it contains the primary codebase");
    process.exit(1);
  }

  // Prevent removal of current worktree
  if (worktree.isCurrent) {
    error('cannot remove current session');
    instruction(
      "Switch to another session first, then remove this one:",
      [
        "cd ~/your-project  # switch to main",
        `claudectl rm ${sessionName}  # then remove`
      ]
    );
    process.exit(1);
  }

  blank();

  // Show warning for uncommitted changes with force
  if (worktree.isClean === false && options.force) {
    warning("Forcing removal of session with uncommitted changes");
    blank();
  }

  // Stop Claude Code session if it exists
  const claudeSession = ClaudeSessionManager.getSession(sessionName);
  if (claudeSession) {
    try {
      info(`Stopping Claude Code session for "${sessionName}"`);
      await ClaudeSessionManager.stopSession(sessionName);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      warning(`Failed to stop Claude Code session: ${errorMessage}`);
    }
  }

  // Remove the worktree
  try {
    const removedWorktree = removeWorktreeByName(sessionName, projectConfig.name, currentDir, options.force || false);
    
    success(`Session "${sessionName}" removed successfully`);
    info(`Removed directory: ${displayPath}`);
    info(`Removed branch: ${removedWorktree.branch}`);
    if (claudeSession) {
      info(`Stopped Claude Code session`);
    }
    
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : String(err);
    fatal(`failed to remove session: ${errorMessage}`);
  }

  blank();
  info("List remaining sessions with: claudectl list");
};