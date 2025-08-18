import { execSync } from "node:child_process";
import { mkdir, mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import {
  createWorktree,
  getCurrentBranch,
  getDefaultBranch,
  isGitRepository,
  listWorktrees,
  removeWorktree,
  updateBranch,
} from "./git";

describe("git utilities", () => {
  let tempDir: string;

  beforeEach(async () => {
    tempDir = await mkdtemp(join(tmpdir(), "claudectl-test-"));
  });

  afterEach(async () => {
    if (tempDir) {
      await rm(tempDir, { recursive: true, force: true });
    }
  });

  describe("isGitRepository", () => {
    it("should return true for a directory with .git folder", async () => {
      // Create a .git directory
      await mkdir(join(tempDir, ".git"));

      const result = isGitRepository(tempDir);
      expect(result).toBe(true);
    });

    it("should return false for a directory without .git folder", () => {
      const result = isGitRepository(tempDir);
      expect(result).toBe(false);
    });

    it("should return true for a real git repository", async () => {
      // Initialize a real git repository
      execSync("git init", { cwd: tempDir, stdio: "pipe" });

      const result = isGitRepository(tempDir);
      expect(result).toBe(true);
    });

    it("should return false for a non-existent directory", () => {
      const result = isGitRepository("/non/existent/path");
      expect(result).toBe(false);
    });

    it("should use current working directory when no path provided", () => {
      // This test assumes we're running from a git repository (which we are)
      const result = isGitRepository();
      expect(result).toBe(true);
    });

    it("should handle permission errors gracefully", () => {
      // Test with a path that would cause permission issues
      const result = isGitRepository("/root/.ssh");
      expect(typeof result).toBe("boolean");
    });
  });

  describe("getCurrentBranch", () => {
    beforeEach(async () => {
      // Initialize a git repository for branch tests
      execSync("git init", { cwd: tempDir, stdio: "pipe" });
      execSync('git config user.name "Test User"', {
        cwd: tempDir,
        stdio: "pipe",
      });
      execSync('git config user.email "test@example.com"', {
        cwd: tempDir,
        stdio: "pipe",
      });

      // Create an initial commit
      execSync('echo "test" > test.txt', { cwd: tempDir, stdio: "pipe" });
      execSync("git add test.txt", { cwd: tempDir, stdio: "pipe" });
      execSync('git commit -m "Initial commit"', {
        cwd: tempDir,
        stdio: "pipe",
      });
    });

    it("should return current branch name", () => {
      const branch = getCurrentBranch(tempDir);
      expect(typeof branch).toBe("string");
      expect(branch.length).toBeGreaterThan(0);
    });

    it("should work with custom branch names", () => {
      execSync("git checkout -b feature-branch", {
        cwd: tempDir,
        stdio: "pipe",
      });

      const branch = getCurrentBranch(tempDir);
      expect(branch).toBe("feature-branch");
    });

    it("should throw error for non-git directory", () => {
      const nonGitDir = join(tempDir, "not-git");

      expect(() => getCurrentBranch(nonGitDir)).toThrow(
        "Failed to get current branch"
      );
    });
  });

  describe("getDefaultBranch", () => {
    beforeEach(async () => {
      // Initialize a git repository
      execSync("git init", { cwd: tempDir, stdio: "pipe" });
      execSync('git config user.name "Test User"', {
        cwd: tempDir,
        stdio: "pipe",
      });
      execSync('git config user.email "test@example.com"', {
        cwd: tempDir,
        stdio: "pipe",
      });

      // Create an initial commit
      execSync('echo "test" > test.txt', { cwd: tempDir, stdio: "pipe" });
      execSync("git add test.txt", { cwd: tempDir, stdio: "pipe" });
      execSync('git commit -m "Initial commit"', {
        cwd: tempDir,
        stdio: "pipe",
      });
    });

    it("should detect main branch when it exists", () => {
      // Rename default branch to main
      execSync("git branch -m main", { cwd: tempDir, stdio: "pipe" });

      const defaultBranch = getDefaultBranch(tempDir);
      expect(defaultBranch).toBe("main");
    });

    it("should detect master branch when main does not exist", () => {
      // Rename default branch to master
      execSync("git branch -m master", { cwd: tempDir, stdio: "pipe" });

      const defaultBranch = getDefaultBranch(tempDir);
      expect(defaultBranch).toBe("master");
    });

    it("should throw error when neither main nor master exists", () => {
      // Rename to something else
      execSync("git branch -m custom-branch", { cwd: tempDir, stdio: "pipe" });

      expect(() => getDefaultBranch(tempDir)).toThrow(
        "Cannot find main or master branch"
      );
    });

    it("should throw error for non-git directory", () => {
      const nonGitDir = join(tempDir, "not-git");

      expect(() => getDefaultBranch(nonGitDir)).toThrow(
        "Failed to determine default branch"
      );
    });
  });

  describe("updateBranch", () => {
    beforeEach(async () => {
      // Initialize a git repository
      execSync("git init", { cwd: tempDir, stdio: "pipe" });
      execSync('git config user.name "Test User"', {
        cwd: tempDir,
        stdio: "pipe",
      });
      execSync('git config user.email "test@example.com"', {
        cwd: tempDir,
        stdio: "pipe",
      });

      // Create an initial commit on main
      execSync("git checkout -b main", { cwd: tempDir, stdio: "pipe" });
      execSync('echo "test" > test.txt', { cwd: tempDir, stdio: "pipe" });
      execSync("git add test.txt", { cwd: tempDir, stdio: "pipe" });
      execSync('git commit -m "Initial commit"', {
        cwd: tempDir,
        stdio: "pipe",
      });
    });

    it("should throw error when no remote origin exists", () => {
      expect(() => updateBranch(tempDir)).toThrow("Failed to update branch");
    });

    it("should throw error for non-git directory", () => {
      const nonGitDir = join(tempDir, "not-git");

      expect(() => updateBranch(nonGitDir)).toThrow("Failed to update branch");
    });
  });

  describe("worktree operations", () => {
    let repoDir: string;
    let worktreeDir: string;

    beforeEach(async () => {
      // Create separate directories for repo and worktree
      repoDir = join(tempDir, "repo");
      worktreeDir = join(tempDir, "worktrees");

      await mkdir(repoDir);
      await mkdir(worktreeDir);

      // Initialize a git repository
      execSync("git init", { cwd: repoDir, stdio: "pipe" });
      execSync('git config user.name "Test User"', {
        cwd: repoDir,
        stdio: "pipe",
      });
      execSync('git config user.email "test@example.com"', {
        cwd: repoDir,
        stdio: "pipe",
      });

      // Create an initial commit on main
      execSync("git checkout -b main", { cwd: repoDir, stdio: "pipe" });
      execSync('echo "test" > test.txt', { cwd: repoDir, stdio: "pipe" });
      execSync("git add test.txt", { cwd: repoDir, stdio: "pipe" });
      execSync('git commit -m "Initial commit"', {
        cwd: repoDir,
        stdio: "pipe",
      });
    });

    describe("createWorktree", () => {
      it("should throw error when no remote origin exists", () => {
        const worktreePath = join(worktreeDir, "feature-1");

        expect(() =>
          createWorktree(worktreePath, "feature-1", "main", repoDir)
        ).toThrow("Failed to create worktree");
      });

      it("should throw error for non-git directory", () => {
        const nonGitDir = join(tempDir, "not-git");
        const worktreePath = join(worktreeDir, "feature-1");

        expect(() =>
          createWorktree(worktreePath, "feature-1", "main", nonGitDir)
        ).toThrow("Failed to create worktree");
      });
    });

    describe("listWorktrees", () => {
      it("should list current worktree", () => {
        const worktrees = listWorktrees(repoDir);

        expect(Array.isArray(worktrees)).toBe(true);
        expect(worktrees.length).toBeGreaterThan(0);
        expect(worktrees[0]).toHaveProperty("path");
        expect(worktrees[0]).toHaveProperty("branch");
        expect(worktrees[0]).toHaveProperty("commit");
      });

      it("should throw error for non-git directory", () => {
        const nonGitDir = join(tempDir, "not-git");

        expect(() => listWorktrees(nonGitDir)).toThrow(
          "Failed to list worktrees"
        );
      });
    });

    describe("removeWorktree", () => {
      it("should throw error when worktree does not exist", () => {
        const nonExistentPath = join(worktreeDir, "non-existent");

        expect(() => removeWorktree(nonExistentPath, repoDir)).toThrow(
          "Failed to remove worktree"
        );
      });

      it("should throw error for non-git directory", () => {
        const nonGitDir = join(tempDir, "not-git");
        const worktreePath = join(worktreeDir, "feature-1");

        expect(() => removeWorktree(worktreePath, nonGitDir)).toThrow(
          "Failed to remove worktree"
        );
      });
    });
  });
});
