/**
 * ClaudeCtl utility functions organized by domain.
 *
 * This module re-exports all utility functions from their respective domain files
 * to provide a clean, organized API while maintaining backward compatibility.
 */

// Git utilities
export {
  isGitRepository,
  getCurrentBranch,
  getDefaultBranch,
  updateBranch,
  createWorktree,
  listWorktrees,
  removeWorktree,
  getProjectWorktrees,
  getWorktreeName,
  findWorktreeByName,
  removeWorktreeByName,
  type WorktreeInfo,
} from "./git.js";

// Directory management utilities
export {
  getGlobalClaudectlDir,
  getProjectsDir,
  getProjectDir,
  ensureDirectory,
  hasClaudectlConfig,
} from "./directories.js";

// Project configuration utilities
export {
  createProjectConfig,
  loadProjectConfig,
  updateProjectConfig,
} from "./config.js";

// Multi-step initialization utilities
export {
  performMultiStepInit,
  type InitStepResult,
  type InitStepError,
  type InitStepOutcome,
} from "./initialization.js";

// Name generation utilities
export {
  generateRandomName,
} from "./naming.js";
