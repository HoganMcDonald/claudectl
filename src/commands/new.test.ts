import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { newCommand } from "./new";

// Mock all utilities and output functions
vi.mock("../utils", () => ({
  isGitRepository: vi.fn(),
  hasClaudectlConfig: vi.fn(),
  loadProjectConfig: vi.fn(),
  getProjectDir: vi.fn(),
  createWorktree: vi.fn(),
  getDefaultBranch: vi.fn(),
  generateRandomName: vi.fn(),
}));

vi.mock("../output", () => ({
  error: vi.fn(),
  info: vi.fn(),
  success: vi.fn(),
  indentedSuccess: vi.fn(),
  instruction: vi.fn(),
  step: vi.fn(),
  blank: vi.fn(),
  section: vi.fn(),
  fatal: vi.fn(),
}));

// Import the mocked functions
const {
  isGitRepository,
  hasClaudectlConfig,
  loadProjectConfig,
  getProjectDir,
  createWorktree,
  getDefaultBranch,
  generateRandomName,
} = await import("../utils");
const {
  error,
  info,
  success,
  indentedSuccess,
  instruction,
  step,
  section,
  fatal,
} = await import("../output");

describe("new command", () => {
  let tempDir: string;
  let originalCwd: string;
  let processExitSpy: ReturnType<typeof vi.spyOn>;

  beforeEach(async () => {
    tempDir = await mkdtemp(join(tmpdir(), "claudectl-test-"));
    originalCwd = process.cwd();

    // Mock process.cwd to return our temp directory
    vi.spyOn(process, "cwd").mockReturnValue(tempDir);

    // Mock process.exit to throw instead of actually exiting
    processExitSpy = vi.spyOn(process, "exit").mockImplementation(() => {
      throw new Error("process.exit() was called");
    });

    // Mock fatal to throw instead of calling process.exit
    vi.mocked(fatal).mockImplementation((message: string) => {
      throw new Error(`fatal: ${message}`);
    });

    // Reset all mocks
    vi.clearAllMocks();
  });

  afterEach(async () => {
    if (tempDir) {
      await rm(tempDir, { recursive: true, force: true });
    }
    vi.restoreAllMocks();
    process.chdir(originalCwd);
  });

  describe("successful worktree creation", () => {
    it("should create worktree with auto-generated name", () => {
      // Setup: git repo, claudectl config exists, successful worktree creation
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: "test-project" });
      vi.mocked(getProjectDir).mockReturnValue(
        "/home/.claudectl/projects/test-project"
      );
      vi.mocked(getDefaultBranch).mockReturnValue("main");
      vi.mocked(generateRandomName).mockReturnValue("brave-penguin");
      vi.mocked(createWorktree).mockImplementation(() => {});

      newCommand();

      // Should load project config
      expect(loadProjectConfig).toHaveBeenCalledWith(tempDir);

      // Should get project directory
      expect(getProjectDir).toHaveBeenCalledWith("test-project");

      // Should get default branch
      expect(getDefaultBranch).toHaveBeenCalledWith(tempDir);

      // Should show section header with auto-generated name
      expect(section).toHaveBeenCalledWith(
        expect.stringContaining("Creating new worktree")
      );

      // Should show steps
      expect(step).toHaveBeenCalledWith(
        1,
        2,
        "Fetching latest main from origin"
      );
      expect(step).toHaveBeenCalledWith(
        2,
        2,
        expect.stringContaining("Creating worktree with branch")
      );

      // Should create worktree with auto-generated name
      expect(createWorktree).toHaveBeenCalledWith(
        "/home/.claudectl/projects/test-project/brave-penguin",
        "brave-penguin",
        "main",
        tempDir
      );

      // Should show success messages
      expect(indentedSuccess).toHaveBeenCalledWith(
        expect.stringContaining("Worktree created at")
      );
      expect(indentedSuccess).toHaveBeenCalledWith(
        expect.stringContaining("Branch")
      );
      expect(success).toHaveBeenCalledWith(
        expect.stringContaining("created successfully")
      );
      expect(info).toHaveBeenCalledWith(
        expect.stringContaining("Switch to the worktree")
      );
    });

    it("should create worktree with provided name", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: "test-project" });
      vi.mocked(getProjectDir).mockReturnValue(
        "/home/.claudectl/projects/test-project"
      );
      vi.mocked(getDefaultBranch).mockReturnValue("main");
      vi.mocked(createWorktree).mockImplementation(() => {});

      newCommand("feature-branch");

      // Should create worktree with provided name
      expect(createWorktree).toHaveBeenCalledWith(
        "/home/.claudectl/projects/test-project/feature-branch",
        "feature-branch",
        "main",
        tempDir
      );

      expect(section).toHaveBeenCalledWith(
        'Creating new worktree "feature-branch"'
      );
      expect(success).toHaveBeenCalledWith(
        'Worktree "feature-branch" created successfully'
      );
    });

    it("should work with master as default branch", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: "test-project" });
      vi.mocked(getProjectDir).mockReturnValue(
        "/home/.claudectl/projects/test-project"
      );
      vi.mocked(getDefaultBranch).mockReturnValue("master");
      vi.mocked(createWorktree).mockImplementation(() => {});

      newCommand("test-branch");

      expect(step).toHaveBeenCalledWith(
        1,
        2,
        "Fetching latest master from origin"
      );
      expect(createWorktree).toHaveBeenCalledWith(
        "/home/.claudectl/projects/test-project/test-branch",
        "test-branch",
        "master",
        tempDir
      );
    });
  });

  describe("error conditions", () => {
    it("should exit if current directory is not a git repository", () => {
      vi.mocked(isGitRepository).mockReturnValue(false);

      expect(() => newCommand()).toThrow("process.exit() was called");

      expect(error).toHaveBeenCalledWith(
        "current directory is not a git repository"
      );
      expect(instruction).toHaveBeenCalledWith(
        "ClaudeCtl requires a git repository. Please initialize one first:",
        ["git init", "git add .", 'git commit -m "Initial commit"']
      );
      expect(processExitSpy).toHaveBeenCalledWith(1);

      // Should not proceed with worktree creation
      expect(createWorktree).not.toHaveBeenCalled();
    });

    it("should exit if current directory is not a claudectl project", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);

      expect(() => newCommand()).toThrow("process.exit() was called");

      expect(error).toHaveBeenCalledWith(
        "current directory is not a claudectl project"
      );
      expect(instruction).toHaveBeenCalledWith(
        "Please initialize a claudectl project first:",
        ["claudectl init"]
      );
      expect(processExitSpy).toHaveBeenCalledWith(1);

      // Should not proceed with worktree creation
      expect(createWorktree).not.toHaveBeenCalled();
    });

    it("should handle project config loading failure", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockImplementation(() => {
        throw new Error("Config not found");
      });

      expect(() => newCommand()).toThrow(
        "fatal: failed to load project configuration"
      );

      expect(fatal).toHaveBeenCalledWith(
        "failed to load project configuration"
      );

      // Should not proceed with worktree creation
      expect(createWorktree).not.toHaveBeenCalled();
    });

    it("should handle default branch detection failure", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: "test-project" });
      vi.mocked(getProjectDir).mockReturnValue(
        "/home/.claudectl/projects/test-project"
      );
      vi.mocked(generateRandomName).mockReturnValue("swift-fox");
      vi.mocked(getDefaultBranch).mockImplementation(() => {
        throw new Error("No default branch found");
      });

      expect(() => newCommand()).toThrow(
        "fatal: failed to determine default branch: No default branch found"
      );

      expect(fatal).toHaveBeenCalledWith(
        "failed to determine default branch: No default branch found"
      );

      // Should not proceed with worktree creation
      expect(createWorktree).not.toHaveBeenCalled();
    });

    it("should handle worktree creation failure", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: "test-project" });
      vi.mocked(getProjectDir).mockReturnValue(
        "/home/.claudectl/projects/test-project"
      );
      vi.mocked(getDefaultBranch).mockReturnValue("main");
      vi.mocked(createWorktree).mockImplementation(() => {
        throw new Error("Worktree creation failed");
      });

      expect(() => newCommand("test-branch")).toThrow(
        "fatal: failed to create worktree: Worktree creation failed"
      );

      expect(fatal).toHaveBeenCalledWith(
        "failed to create worktree: Worktree creation failed"
      );
    });
  });

  describe("auto-generated name format", () => {
    it("should generate names in adjective-animal format", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: "test-project" });
      vi.mocked(getProjectDir).mockReturnValue(
        "/home/.claudectl/projects/test-project"
      );
      vi.mocked(getDefaultBranch).mockReturnValue("main");
      vi.mocked(generateRandomName).mockReturnValue("clever-dolphin");
      vi.mocked(createWorktree).mockImplementation(() => {});

      newCommand();

      // Check that createWorktree was called with a properly formatted name
      expect(createWorktree).toHaveBeenCalledWith(
        "/home/.claudectl/projects/test-project/clever-dolphin",
        "clever-dolphin",
        "main",
        tempDir
      );
    });

    it("should generate friendly adjective-animal names", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: "test-project" });
      vi.mocked(getProjectDir).mockReturnValue(
        "/home/.claudectl/projects/test-project"
      );
      vi.mocked(getDefaultBranch).mockReturnValue("main");
      vi.mocked(generateRandomName).mockReturnValue("golden-eagle");
      vi.mocked(createWorktree).mockImplementation(() => {});

      newCommand();

      // Check that createWorktree was called with adjective-animal pattern
      const actualBranchName = vi.mocked(createWorktree).mock.calls[0][1];
      expect(actualBranchName).toBe("golden-eagle");
      expect(actualBranchName).toMatch(/^[a-z]+-[a-z]+$/);

      const parts = actualBranchName.split("-");
      expect(parts).toHaveLength(2);
      expect(parts[0]).toBe("golden");
      expect(parts[1]).toBe("eagle");
    });
  });

  describe("integration with utilities", () => {
    it("should pass correct parameters to utility functions", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: "my-project" });
      vi.mocked(getProjectDir).mockReturnValue(
        "/custom/path/projects/my-project"
      );
      vi.mocked(getDefaultBranch).mockReturnValue("main");
      vi.mocked(createWorktree).mockImplementation(() => {});

      newCommand("my-feature");

      expect(isGitRepository).toHaveBeenCalledWith(tempDir);
      expect(hasClaudectlConfig).toHaveBeenCalledWith(tempDir);
      expect(loadProjectConfig).toHaveBeenCalledWith(tempDir);
      expect(getProjectDir).toHaveBeenCalledWith("my-project");
      expect(getDefaultBranch).toHaveBeenCalledWith(tempDir);
      expect(createWorktree).toHaveBeenCalledWith(
        "/custom/path/projects/my-project/my-feature",
        "my-feature",
        "main",
        tempDir
      );
    });
  });
});
