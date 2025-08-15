import * as fs from "node:fs";
import * as path from "node:path";

/**
 * Git-related utility functions for ClaudeCtl.
 *
 * This module handles git repository detection and validation.
 */

/**
 * Checks if a directory is a git repository by looking for a .git directory.
 *
 * @param dirPath - The directory path to check. Defaults to current working directory.
 * @returns True if the directory contains a .git folder, false otherwise.
 *
 * @example
 * ```typescript
 * if (isGitRepository()) {
 *   console.log("Current directory is a git repository");
 * }
 *
 * if (isGitRepository("/path/to/project")) {
 *   console.log("Project directory is a git repository");
 * }
 * ```
 */
export function isGitRepository(dirPath: string = process.cwd()): boolean {
  try {
    const gitPath = path.join(dirPath, ".git");
    return fs.existsSync(gitPath);
  } catch {
    return false;
  }
}

