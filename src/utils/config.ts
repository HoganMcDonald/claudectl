import * as fs from "node:fs";
import * as path from "node:path";
import { type ProjectConfig, ProjectConfigSchema } from "../types.js";

/**
 * Project configuration utilities for ClaudeCtl.
 *
 * This module handles all project configuration operations including
 * creating, loading, updating, and validating config files using Zod schemas.
 */

/**
 * Creates the .claudectl directory and config.json file with validated configuration.
 *
 * @param projectPath - The absolute path to the project directory.
 * @param config - The project configuration object to save.
 * @throws {ZodError} When the config object fails validation.
 * @throws {Error} When file system operations fail.
 *
 * @example
 * ```typescript
 * const config = { name: "my-project" };
 * createProjectConfig("/path/to/project", config);
 * ```
 */
export function createProjectConfig(
  projectPath: string,
  config: ProjectConfig
): void {
  const claudectlDir = path.join(projectPath, ".claudectl");
  const configPath = path.join(claudectlDir, "config.json");

  const validatedConfig = ProjectConfigSchema.parse(config);

  if (!fs.existsSync(claudectlDir)) {
    fs.mkdirSync(claudectlDir, { recursive: true });
  }

  fs.writeFileSync(configPath, JSON.stringify(validatedConfig, null, 2));
}

/**
 * Loads and validates the project configuration from .claudectl/config.json.
 *
 * @param projectPath - The project directory path. Defaults to current working directory.
 * @returns The validated project configuration object.
 * @throws {Error} When the config file doesn't exist.
 * @throws {ZodError} When the config file contains invalid data.
 * @throws {SyntaxError} When the config file contains invalid JSON.
 *
 * @example
 * ```typescript
 * try {
 *   const config = loadProjectConfig();
 *   console.log(`Project name: ${config.name}`);
 * } catch (error) {
 *   console.error("Failed to load project config:", error.message);
 * }
 * ```
 */
export function loadProjectConfig(
  projectPath: string = process.cwd()
): ProjectConfig {
  const configPath = path.join(projectPath, ".claudectl", "config.json");

  if (!fs.existsSync(configPath)) {
    throw new Error("No claudectl config found. Run 'claudectl init' first.");
  }

  const configContent = fs.readFileSync(configPath, "utf-8");
  const parsedConfig = JSON.parse(configContent);

  return ProjectConfigSchema.parse(parsedConfig);
}

/**
 * Updates the project configuration file with new values.
 *
 * @param projectPath - The absolute path to the project directory.
 * @param updates - Partial configuration object with properties to update.
 * @throws {Error} When the existing config file cannot be loaded.
 * @throws {ZodError} When the merged config fails validation.
 *
 * @example
 * ```typescript
 * // Update just the project name
 * updateProjectConfig("/path/to/project", { name: "new-project-name" });
 * ```
 */
export function updateProjectConfig(
  projectPath: string,
  updates: Partial<ProjectConfig>
): void {
  const currentConfig = loadProjectConfig(projectPath);
  const updatedConfig = { ...currentConfig, ...updates };
  createProjectConfig(projectPath, updatedConfig);
}
