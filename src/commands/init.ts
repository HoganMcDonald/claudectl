import * as path from "node:path";
import {
  blank,
  error,
  indentedError,
  indentedSuccess,
  info,
  instruction,
  section,
  step,
  success,
} from "../output.js";
import {
  hasClaudectlConfig,
  isGitRepository,
  performMultiStepInit,
} from "../utils/index.js";

export const initCommand = (projectName?: string): void => {
  const currentDir = process.cwd();
  const resolvedProjectName = projectName ?? path.basename(currentDir);

  // Check if current directory is a git repository
  if (!isGitRepository(currentDir)) {
    error("current directory is not a git repository");
    instruction(
      "ClaudeCtl uses git worktrees for managing code contexts. Please initialize a git repository first:",
      ["git init", "git add .", 'git commit -m "Initial commit"']
    );
    process.exit(1);
  }

  // Check if project is already initialized
  if (hasClaudectlConfig(currentDir)) {
    info(`Project "${resolvedProjectName}" is already initialized`);
    return;
  }

  // Perform multi-step initialization
  section(`Initializing ClaudeCtl project "${resolvedProjectName}"`);
  blank();

  const results = performMultiStepInit(resolvedProjectName, currentDir);
  let hasErrors = false;

  results.forEach((result, index) => {
    const stepNumber = index + 1;
    const totalSteps = results.length;

    if (result.success) {
      step(stepNumber, totalSteps, result.message);
      if (result.details) {
        indentedSuccess(result.details);
      }
    } else {
      step(stepNumber, totalSteps, result.error);
      if (result.details) {
        indentedError(result.details);
      }
      hasErrors = true;
    }
  });

  blank();

  if (hasErrors) {
    const completedSteps = results.filter((r) => r.success).length;
    error(
      `Initialization partially completed (${completedSteps}/${results.length} steps successful)`
    );
    process.exit(1);
  } else {
    success(
      `ClaudeCtl project "${resolvedProjectName}" initialized successfully`
    );
    info("You can now use ClaudeCtl commands in this project");
  }
};
