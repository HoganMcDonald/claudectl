import { error, instruction, fatal } from "../output.js";
import {
  isGitRepository,
  hasClaudectlConfig,
  loadProjectConfig,
} from "./index.js";

/**
 * Common error handling utilities for CLI commands
 */
export function handleProjectValidation(currentDir: string, projectName?: string): { name: string } {

  // Check if current directory is a git repository
  if (!isGitRepository(currentDir)) {
    error("current directory is not a git repository");
    instruction(
      "ClaudeCtl requires a git repository. Please navigate to one:",
      ["cd /path/to/your/git/project", projectName ? `claudectl ${projectName}` : "claudectl list"]
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
  try {
    return loadProjectConfig(currentDir);
  } catch (_err) {
    fatal("failed to load project configuration");
  }
}

export function validateSessionName(sessionName: string, operation: string): void {
  if (!sessionName || sessionName.trim().length === 0) {
    error("session name is required");
    instruction(
      `Specify the name of the session to ${operation}:`,
      [`claudectl ${operation} brave-penguin`, `claudectl ${operation} swift-fox`]
    );
    process.exit(1);
  }
}