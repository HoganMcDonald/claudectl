/**
 * ClaudeCtl utility functions organized by domain.
 *
 * This module re-exports all utility functions from their respective domain files
 * to provide a clean, organized API while maintaining backward compatibility.
 */

// Project configuration utilities
export {
  createProjectConfig,
  loadProjectConfig,
  updateProjectConfig,
} from "./config.js";

// Directory management utilities
export {
  ensureDirectory,
  getGlobalClaudectlDir,
  getProjectDir,
  getProjectsDir,
  hasClaudectlConfig,
} from "./directories.js";
// Git utilities
export {
  createWorktree,
  findWorktreeByName,
  getCurrentBranch,
  getDefaultBranch,
  getProjectWorktrees,
  getWorktreeName,
  isGitRepository,
  listWorktrees,
  removeWorktree,
  removeWorktreeByName,
  updateBranch,
  type WorktreeInfo,
} from "./git.js";

// Multi-step initialization utilities
export {
  type InitStepError,
  type InitStepOutcome,
  type InitStepResult,
  performMultiStepInit,
} from "./initialization.js";

// Name generation utilities
export { generateRandomName } from "./naming.js";
