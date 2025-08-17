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
 * Lists all git worktrees in the repository.
 *
 * @param repoPath - The repository path. Defaults to current working directory.
 * @returns Array of worktree information objects.
 * @throws {Error} When git command fails.
 *
 * @example
 * ```typescript
 * const worktrees = listWorktrees();
 * worktrees.forEach(wt => console.log(`${wt.branch}: ${wt.path}`));
 * ```
 */
export function listWorktrees(repoPath: string = process.cwd()): Array<{
  path: string;
  branch: string;
  commit: string;
}> {
  try {
    const result = execSync("git worktree list --porcelain", {
      cwd: repoPath,
      encoding: "utf-8",
      stdio: "pipe",
    });

    const worktrees: Array<{ path: string; branch: string; commit: string }> = [];
    const lines = result.trim().split("\n");
    
    let currentWorktree: Partial<{ path: string; branch: string; commit: string }> = {};
    
    for (const line of lines) {
      if (line.startsWith("worktree ")) {
        if (Object.keys(currentWorktree).length > 0) {
          worktrees.push(currentWorktree as { path: string; branch: string; commit: string });
        }
        currentWorktree = { path: line.substring(9) };
      } else if (line.startsWith("HEAD ")) {
        currentWorktree.commit = line.substring(5);
      } else if (line.startsWith("branch ")) {
        currentWorktree.branch = line.substring(7);
      }
    }
    
    if (Object.keys(currentWorktree).length > 0) {
      worktrees.push(currentWorktree as { path: string; branch: string; commit: string });
    }

    return worktrees;
  } catch (error) {
    throw new Error(`Failed to list worktrees: ${error}`);
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

