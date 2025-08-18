import * as path from "node:path";
import {
  type ClaudeSessionInfo,
  ClaudeSessionManager,
} from "../claude-session.js";
import {
  blank,
  dim,
  emphasis,
  info,
  section,
  success,
  table,
} from "../output.js";
import { handleProjectValidation } from "../utils/errors.js";
import {
  getProjectWorktrees,
  getWorktreeName,
  type WorktreeInfo,
} from "../utils.js";

/**
 * Formats a commit hash for display (short version).
 */
function formatCommitHash(commit: string): string {
  return commit.substring(0, 7);
}

/**
 * Formats the status column for a task.
 */
function formatStatus(task: WorktreeInfo, session?: ClaudeSessionInfo): string {
  const parts: string[] = [];

  if (task.isCurrent) {
    parts.push("current");
  }

  if (task.isClean === true) {
    parts.push("clean");
  } else if (task.isClean === false) {
    parts.push("dirty");
  }

  if (session) {
    parts.push("claude");
  }

  return parts.join(", ") || "-";
}

/**
 * Formats commit message for display (truncated if too long).
 */
function formatCommitMessage(
  message: string | undefined,
  maxLength: number = 50
): string {
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
export const listCommand = async (): Promise<void> => {
  const currentDir = process.cwd();
  const projectConfig = handleProjectValidation(currentDir);

  // Get all tasks for this project (excluding main repository)
  let allTasks: WorktreeInfo[];
  try {
    allTasks = getProjectWorktrees(projectConfig.name, currentDir);
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to list tasks: ${errorMessage}`);
  }

  // Filter out the main repository - only show created sessions
  const tasks = allTasks.filter((task) => !task.isMain);

  // Clean up dead sessions and get active ones
  ClaudeSessionManager.cleanupSessions();
  const sessions = ClaudeSessionManager.listSessions();
  const sessionMap = new Map<string, ClaudeSessionInfo>();

  // Map sessions by worktree name
  for (const session of sessions) {
    sessionMap.set(session.sessionName, session);
  }

  // Display results
  section(`Sessions for project "${projectConfig.name}"`);
  blank();

  if (tasks.length === 0) {
    info("No sessions found for this project");
    blank();
    info("Create a new session with: claudectl new [name]");
    return;
  }

  // Prepare table data
  const headers = ["Name", "Branch", "Commit", "Status", "Last Commit"];
  const rows = tasks.map((task) => {
    const taskName = getTaskDisplayName(task, projectConfig.name);
    const session = sessionMap.get(taskName);

    return [
      taskName,
      task.branch || "-",
      formatCommitHash(task.commit),
      formatStatus(task, session),
      formatCommitMessage(task.commitMessage),
    ];
  });

  // Display table
  table(headers, rows);

  // Show current session info (if in a session)
  const currentTask = tasks.find((task) => task.isCurrent);
  if (currentTask) {
    blank();
    const currentName = getTaskDisplayName(currentTask, projectConfig.name);
    success(`Currently in session: ${currentName}`);
  }

  // Show switch instructions
  blank();
  emphasis("Switch to a session:");
  tasks.forEach((task) => {
    if (!task.isCurrent) {
      const displayName = getTaskDisplayName(task, projectConfig.name);
      dim(`  cd ${task.path}  # ${displayName}`);
    }
  });
};
