import {
  isGitRepository,
  hasClaudectlConfig,
  loadProjectConfig,
  findWorktreeByName,
  removeWorktreeByName,
  getWorktreeName,
  type WorktreeInfo,
} from "../utils";
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
} from "../output";

/**
 * Removes a worktree/session by name from the current claudectl project.
 *
 * @param sessionName - Name of the session/worktree to remove.
 * @param options - Command options including force flag.
 */
export const rmCommand = (sessionName: string, options: { force?: boolean } = {}): void => {
  const currentDir = process.cwd();

  // Validate session name is provided
  if (!sessionName || sessionName.trim().length === 0) {
    error("session name is required");
    instruction(
      "Specify the name of the session to remove:",
      ["claudectl rm brave-penguin", "claudectl rm swift-fox"]
    );
    process.exit(1);
  }

  // Check if current directory is a git repository
  if (!isGitRepository(currentDir)) {
    error("current directory is not a git repository");
    instruction(
      "ClaudeCtl requires a git repository. Please navigate to one:",
      ["cd /path/to/your/git/project", `claudectl rm ${sessionName}`]
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

  // Find the worktree
  let worktree: WorktreeInfo | null;
  try {
    worktree = findWorktreeByName(sessionName, projectConfig.name, currentDir);
  } catch (err) {
    fatal(`failed to find session: ${err instanceof Error ? err.message : String(err)}`);
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

  // Remove the worktree
  try {
    const removedWorktree = removeWorktreeByName(sessionName, projectConfig.name, currentDir, options.force || false);
    
    success(`Session "${sessionName}" removed successfully`);
    info(`Removed directory: ${displayPath}`);
    info(`Removed branch: ${removedWorktree.branch}`);
    
  } catch (err) {
    fatal(`failed to remove session: ${err instanceof Error ? err.message : String(err)}`);
  }

  blank();
  info("List remaining sessions with: claudectl list");
};