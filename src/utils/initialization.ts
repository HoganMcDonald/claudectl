import type { ProjectConfig } from "../types";
import { createProjectConfig } from "./config";
import {
  ensureDirectory,
  getGlobalClaudectlDir,
  getProjectsDir,
  getProjectDir,
} from "./directories";

/**
 * Multi-step initialization utilities for ClaudeCtl.
 *
 * This module handles the complete initialization workflow including
 * project setup and directory creation.
 */

/**
 * Result of a successful initialization step.
 */
export interface InitStepResult {
  success: true;
  message: string;
  details?: string;
}

/**
 * Result of a failed initialization step.
 */
export interface InitStepError {
  success: false;
  error: string;
  details?: string;
}

export type InitStepOutcome = InitStepResult | InitStepError;

/**
 * Performs the complete multi-step initialization process for a ClaudeCtl project.
 *
 * @param projectName - Name of the project to initialize.
 * @param projectPath - Path to the project directory (defaults to current working directory).
 * @returns Array of step results indicating success/failure of each step.
 *
 * @example
 * ```typescript
 * const results = performMultiStepInit("my-project");
 * results.forEach((result, i) => {
 *   if (result.success) {
 *     console.log(`Step ${i + 1}: ${result.message}`);
 *   } else {
 *     console.error(`Step ${i + 1} failed: ${result.error}`);
 *   }
 * });
 * ```
 */
export function performMultiStepInit(
  projectName: string,
  projectPath: string = process.cwd()
): InitStepOutcome[] {
  const results: InitStepOutcome[] = [];

  // Step 1: Create local .claudectl directory and config
  try {
    const config: ProjectConfig = { name: projectName };
    createProjectConfig(projectPath, config);
    results.push({
      success: true,
      message: "Project configuration",
      details: ".claudectl/config.json",
    });
  } catch (error) {
    results.push({
      success: false,
      error: "Failed to create project configuration",
      details: error instanceof Error ? error.message : String(error),
    });
    return results; // Stop here if project config fails
  }

  // Step 2: Create global directory structure
  try {
    const globalDir = getGlobalClaudectlDir();
    const projectsDir = getProjectsDir();
    const projectDir = getProjectDir(projectName);

    ensureDirectory(globalDir);
    ensureDirectory(projectsDir);
    ensureDirectory(projectDir);

    results.push({
      success: true,
      message: "Global directory structure",
      details: `~/.claudectl/projects/${projectName}`,
    });
  } catch (error) {
    results.push({
      success: false,
      error: "Failed to create global directory structure",
      details: error instanceof Error ? error.message : String(error),
    });
    return results; // Stop here if directory creation fails
  }

  return results;
}
