import * as fs from "node:fs";
import * as path from "node:path";

/**
 * Directory management utilities for ClaudeCtl.
 *
 * This module handles all directory operations including global directory structure,
 * project directories, and path resolution.
 */

/**
 * Gets the global ClaudeCtl directory path in the user's home directory.
 *
 * @returns The absolute path to ~/.claudectl directory.
 * @throws {Error} When home directory cannot be determined.
 *
 * @example
 * ```typescript
 * const globalDir = getGlobalClaudectlDir();
 * console.log(`Global directory: ${globalDir}`);
 * ```
 */
export function getGlobalClaudectlDir(): string {
  const homeDir = process.env.HOME || process.env.USERPROFILE;
  if (!homeDir) {
    throw new Error("Unable to determine home directory");
  }
  return path.join(homeDir, ".claudectl");
}

/**
 * Gets the projects directory path within the global ClaudeCtl directory.
 *
 * @returns The absolute path to ~/.claudectl/projects directory.
 *
 * @example
 * ```typescript
 * const projectsDir = getProjectsDir();
 * console.log(`Projects directory: ${projectsDir}`);
 * ```
 */
export function getProjectsDir(): string {
  return path.join(getGlobalClaudectlDir(), "projects");
}

/**
 * Gets the project-specific directory path within the global projects directory.
 *
 * @param projectName - Name of the project.
 * @returns The absolute path to ~/.claudectl/projects/[projectName] directory.
 *
 * @example
 * ```typescript
 * const projectDir = getProjectDir("my-project");
 * console.log(`Project directory: ${projectDir}`);
 * ```
 */
export function getProjectDir(projectName: string): string {
  return path.join(getProjectsDir(), projectName);
}

/**
 * Creates a directory if it doesn't exist, including parent directories.
 *
 * @param dirPath - The directory path to create.
 * @throws {Error} When directory creation fails.
 *
 * @example
 * ```typescript
 * ensureDirectory("/path/to/new/directory");
 * ```
 */
export function ensureDirectory(dirPath: string): void {
  if (!fs.existsSync(dirPath)) {
    try {
      fs.mkdirSync(dirPath, { recursive: true });
    } catch (error) {
      throw new Error(`Failed to create directory ${dirPath}: ${error}`);
    }
  }
}

/**
 * Checks if a .claudectl directory already exists in the project.
 *
 * @param dirPath - The project directory path to check. Defaults to current working directory.
 * @returns True if a .claudectl directory exists, false otherwise.
 *
 * @example
 * ```typescript
 * if (hasClaudectlConfig()) {
 *   console.log("Project is already initialized");
 * }
 * ```
 */
export function hasClaudectlConfig(dirPath: string = process.cwd()): boolean {
  const claudectlPath = path.join(dirPath, ".claudectl");
  return fs.existsSync(claudectlPath);
}
