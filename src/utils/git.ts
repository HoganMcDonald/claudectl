import * as fs from "node:fs";
import * as path from "node:path";
import { execSync } from "node:child_process";

/**
 * Git-related utility functions for ClaudeCtl.
 *
 * This module handles git repository detection, validation, and worktree operations.
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

/**
 * Gets the current git branch name.
 *
 * @param repoPath - The repository path. Defaults to current working directory.
 * @returns The current branch name.
 * @throws {Error} When git command fails or not in a git repository.
 *
 * @example
 * ```typescript
 * const branch = getCurrentBranch();
 * console.log(`Current branch: ${branch}`);
 * ```
 */
export function getCurrentBranch(repoPath: string = process.cwd()): string {
  try {
    const result = execSync("git branch --show-current", {
      cwd: repoPath,
      encoding: "utf-8",
      stdio: "pipe",
    });
    return result.trim();
  } catch (error) {
    throw new Error(`Failed to get current branch: ${error}`);
  }
}

/**
 * Gets the default branch name (main or master).
 *
 * @param repoPath - The repository path. Defaults to current working directory.
 * @returns The default branch name (main or master).
 * @throws {Error} When git command fails or cannot determine default branch.
 *
 * @example
 * ```typescript
 * const defaultBranch = getDefaultBranch();
 * console.log(`Default branch: ${defaultBranch}`);
 * ```
 */
export function getDefaultBranch(repoPath: string = process.cwd()): string {
  try {
    // Try to get the default branch from remote
    try {
      const result = execSync("git symbolic-ref refs/remotes/origin/HEAD", {
        cwd: repoPath,
        encoding: "utf-8",
        stdio: "pipe",
      });
      const fullRef = result.trim();
      return fullRef.replace("refs/remotes/origin/", "");
    } catch {
      // Fallback: check if main exists, otherwise use master
      try {
        execSync("git show-ref --verify --quiet refs/heads/main", {
          cwd: repoPath,
          stdio: "pipe",
        });
        return "main";
      } catch {
        try {
          execSync("git show-ref --verify --quiet refs/heads/master", {
            cwd: repoPath,
            stdio: "pipe",
          });
          return "master";
        } catch {
          throw new Error("Cannot find main or master branch");
        }
      }
    }
  } catch (error) {
    throw new Error(`Failed to determine default branch: ${error}`);
  }
}

/**
 * Updates the default branch to the latest from origin.
 *
 * @param repoPath - The repository path. Defaults to current working directory.
 * @param branch - The branch to update. Defaults to the default branch.
 * @throws {Error} When git command fails.
 *
 * @example
 * ```typescript
 * updateBranch(); // Updates default branch
 * updateBranch(process.cwd(), "main"); // Updates main branch specifically
 * ```
 */
export function updateBranch(repoPath: string = process.cwd(), branch?: string): void {
  try {
    const targetBranch = branch || getDefaultBranch(repoPath);
    
    // Fetch latest changes
    execSync("git fetch origin", {
      cwd: repoPath,
      stdio: "pipe",
    });

    // Update the branch
    execSync(`git checkout ${targetBranch}`, {
      cwd: repoPath,
      stdio: "pipe",
    });

    execSync(`git pull origin ${targetBranch}`, {
      cwd: repoPath,
      stdio: "pipe",
    });
  } catch (error) {
    throw new Error(`Failed to update branch: ${error}`);
  }
}

/**
 * Creates a new git worktree at the specified path.
 *
 * @param worktreePath - The path where the worktree will be created.
 * @param branchName - The name for the new branch in the worktree.
 * @param baseBranch - The base branch to create the worktree from. Defaults to default branch.
 * @param repoPath - The repository path. Defaults to current working directory.
 * @throws {Error} When git worktree creation fails.
 *
 * @example
 * ```typescript
 * createWorktree("/path/to/worktree", "feature-branch");
 * createWorktree("/path/to/worktree", "feature-branch", "main");
 * ```
 */
export function createWorktree(
  worktreePath: string,
  branchName: string,
  baseBranch?: string,
  repoPath: string = process.cwd()
): void {
  try {
    const targetBaseBranch = baseBranch || getDefaultBranch(repoPath);
    
    // Ensure we have the latest changes
    execSync("git fetch origin", {
      cwd: repoPath,
      stdio: "pipe",
    });

    // Create the worktree with a new branch based on the latest base branch
    execSync(`git worktree add -b ${branchName} ${worktreePath} origin/${targetBaseBranch}`, {
      cwd: repoPath,
      stdio: "pipe",
    });
  } catch (error) {
    throw new Error(`Failed to create worktree: ${error}`);
  }
}

/**
 * Information about a git worktree.
 */
export interface WorktreeInfo {
  path: string;
  branch: string;
  commit: string;
  isMain: boolean;
  isCurrent: boolean;
  isClean?: boolean;
  commitMessage?: string;
}

/**
 * Lists all git worktrees in the repository with detailed information.
 *
 * @param repoPath - The repository path. Defaults to current working directory.
 * @returns Array of worktree information objects.
 * @throws {Error} When git command fails.
 *
 * @example
 * ```typescript
 * const worktrees = listWorktrees();
 * worktrees.forEach(wt => console.log(`${wt.branch}: ${wt.path} (${wt.isMain ? 'main' : 'worktree'})`));
 * ```
 */
export function listWorktrees(repoPath: string = process.cwd()): WorktreeInfo[] {
  try {
    const result = execSync("git worktree list --porcelain", {
      cwd: repoPath,
      encoding: "utf-8",
      stdio: "pipe",
    });

    const worktrees: WorktreeInfo[] = [];
    const lines = result.trim().split("\n");
    const currentPath = path.resolve(repoPath);
    
    let currentWorktree: Partial<WorktreeInfo> = {};
    
    for (const line of lines) {
      if (line.startsWith("worktree ")) {
        if (Object.keys(currentWorktree).length > 0) {
          worktrees.push(currentWorktree as WorktreeInfo);
        }
        const worktreePath = line.substring(9);
        currentWorktree = { 
          path: worktreePath,
          isMain: false,
          isCurrent: path.resolve(worktreePath) === currentPath
        };
      } else if (line.startsWith("HEAD ")) {
        currentWorktree.commit = line.substring(5);
      } else if (line.startsWith("branch ")) {
        const branchRef = line.substring(7);
        currentWorktree.branch = branchRef.replace("refs/heads/", "");
        // Mark as main if it's the default branch
        currentWorktree.isMain = currentWorktree.branch === "main" || currentWorktree.branch === "master";
      } else if (line.startsWith("bare") || line.startsWith("detached")) {
        // Handle special cases - for now we'll skip these
      }
    }
    
    if (Object.keys(currentWorktree).length > 0) {
      worktrees.push(currentWorktree as WorktreeInfo);
    }

    // Enrich with additional information
    for (const worktree of worktrees) {
      try {
        // Get commit message
        const commitMsg = execSync(`git log -1 --pretty=format:"%s" ${worktree.commit}`, {
          cwd: repoPath,
          encoding: "utf-8",
          stdio: "pipe",
        });
        worktree.commitMessage = commitMsg.trim();

        // Check if worktree is clean (only for existing directories)
        if (fs.existsSync(worktree.path)) {
          try {
            const status = execSync("git status --porcelain", {
              cwd: worktree.path,
              encoding: "utf-8",
              stdio: "pipe",
            });
            worktree.isClean = status.trim().length === 0;
          } catch {
            worktree.isClean = undefined; // Can't determine
          }
        }
      } catch {
        // If we can't get additional info, that's okay
        worktree.commitMessage = undefined;
        worktree.isClean = undefined;
      }
    }

    return worktrees;
  } catch (error) {
    throw new Error(`Failed to list worktrees: ${error}`);
  }
}

/**
 * Gets worktrees that belong to a specific claudectl project.
 *
 * @param projectName - Name of the claudectl project.
 * @param repoPath - The repository path. Defaults to current working directory.
 * @returns Array of worktree information for the project.
 * @throws {Error} When git command fails.
 *
 * @example
 * ```typescript
 * const projectWorktrees = getProjectWorktrees("my-project");
 * console.log(`Found ${projectWorktrees.length} worktrees for my-project`);
 * ```
 */
export function getProjectWorktrees(projectName: string, repoPath: string = process.cwd()): WorktreeInfo[] {
  const { getGlobalClaudectlDir } = require("./directories");
  const allWorktrees = listWorktrees(repoPath);
  const projectPath = path.join(getGlobalClaudectlDir(), "projects", projectName);
  
  return allWorktrees.filter(worktree => {
    // Include main repository worktree and any worktrees in the project directory
    return worktree.isMain || worktree.path.startsWith(projectPath);
  });
}

/**
 * Gets the worktree name from a worktree path.
 *
 * @param worktreePath - The full path to the worktree.
 * @param projectName - Name of the claudectl project.
 * @returns The worktree name or null if not a project worktree.
 *
 * @example
 * ```typescript
 * const name = getWorktreeName("/home/.claudectl/projects/my-project/brave-penguin", "my-project");
 * console.log(name); // "brave-penguin"
 * ```
 */
export function getWorktreeName(worktreePath: string, projectName: string): string | null {
  const { getGlobalClaudectlDir } = require("./directories");
  const projectPath = path.join(getGlobalClaudectlDir(), "projects", projectName);
  
  if (worktreePath.startsWith(projectPath)) {
    const relativePath = path.relative(projectPath, worktreePath);
    // Return the first directory name (the worktree name)
    const parts = relativePath.split(path.sep);
    return parts[0] || null;
  }
  
  return null;
}

/**
 * Finds a worktree by name within a specific project.
 *
 * @param worktreeName - Name of the worktree to find.
 * @param projectName - Name of the claudectl project.
 * @param repoPath - The repository path. Defaults to current working directory.
 * @returns The worktree information or null if not found.
 * @throws {Error} When git command fails.
 *
 * @example
 * ```typescript
 * const worktree = findWorktreeByName("brave-penguin", "my-project");
 * if (worktree) {
 *   console.log(`Found worktree at: ${worktree.path}`);
 * }
 * ```
 */
export function findWorktreeByName(
  worktreeName: string,
  projectName: string,
  repoPath: string = process.cwd()
): WorktreeInfo | null {
  const projectWorktrees = getProjectWorktrees(projectName, repoPath);
  
  return projectWorktrees.find(worktree => {
    if (worktree.isMain && worktreeName === "main") {
      return true;
    }
    
    const name = getWorktreeName(worktree.path, projectName);
    return name === worktreeName;
  }) || null;
}

/**
 * Removes a worktree by name from a specific project.
 *
 * @param worktreeName - Name of the worktree to remove.
 * @param projectName - Name of the claudectl project.
 * @param repoPath - The repository path. Defaults to current working directory.
 * @param force - Whether to force removal even if worktree has uncommitted changes.
 * @returns The removed worktree information.
 * @throws {Error} When worktree is not found, is the main repository, or removal fails.
 *
 * @example
 * ```typescript
 * const removed = removeWorktreeByName("brave-penguin", "my-project");
 * console.log(`Removed worktree: ${removed.path}`);
 * ```
 */
export function removeWorktreeByName(
  worktreeName: string,
  projectName: string,
  repoPath: string = process.cwd(),
  force: boolean = false
): WorktreeInfo {
  // Find the worktree
  const worktree = findWorktreeByName(worktreeName, projectName, repoPath);
  
  if (!worktree) {
    throw new Error(`Worktree "${worktreeName}" not found in project "${projectName}"`);
  }
  
  // Prevent removal of main repository
  if (worktree.isMain) {
    throw new Error('Cannot remove main repository worktree');
  }
  
  // Prevent removal of current worktree
  if (worktree.isCurrent) {
    throw new Error('Cannot remove current worktree. Please switch to another worktree first');
  }
  
  // Check for uncommitted changes unless forced
  if (!force && worktree.isClean === false) {
    throw new Error(`Worktree "${worktreeName}" has uncommitted changes. Use --force to remove anyway`);
  }
  
  // Remove the worktree
  try {
    removeWorktree(worktree.path, repoPath, force);
    return worktree;
  } catch (error) {
    throw new Error(`Failed to remove worktree: ${error}`);
  }
}

/**
 * Removes a git worktree.
 *
 * @param worktreePath - The path of the worktree to remove.
 * @param repoPath - The repository path. Defaults to current working directory.
 * @param force - Whether to force removal. Defaults to false.
 * @throws {Error} When git worktree removal fails.
 *
 * @example
 * ```typescript
 * removeWorktree("/path/to/worktree");
 * removeWorktree("/path/to/worktree", process.cwd(), true); // Force removal
 * ```
 */
export function removeWorktree(
  worktreePath: string,
  repoPath: string = process.cwd(),
  force: boolean = false
): void {
  try {
    const forceFlag = force ? "--force" : "";
    execSync(`git worktree remove ${forceFlag} ${worktreePath}`, {
      cwd: repoPath,
      stdio: "pipe",
    });
  } catch (error) {
    throw new Error(`Failed to remove worktree: ${error}`);
  }
}

