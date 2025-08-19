import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

// Mock all dependencies
vi.mock("../output.js", () => ({
  blank: vi.fn(),
  error: vi.fn(),
  indentedError: vi.fn(),
  indentedSuccess: vi.fn(),
  info: vi.fn(),
  instruction: vi.fn(),
  section: vi.fn(),
  step: vi.fn(),
  success: vi.fn(),
}));

vi.mock("../utils/index.js", () => ({
  hasClaudectlConfig: vi.fn(),
  isGitRepository: vi.fn(),
  performMultiStepInit: vi.fn(),
}));

// Import after mocking
const { initCommand } = await import("./init.js");
const {
  hasClaudectlConfig,
  isGitRepository,
  performMultiStepInit,
} = await import("../utils/index.js");
const { error, instruction, success, info } = await import("../output.js");

describe("init command", () => {
  let tempDir: string;
  let originalCwd: string;
  let _mockProcessExit: ReturnType<typeof vi.spyOn>;

  beforeEach(async () => {
    tempDir = await mkdtemp(join(tmpdir(), "claudectl-test-"));
    originalCwd = process.cwd();

    // Mock process.cwd to return our temp directory
    vi.spyOn(process, "cwd").mockReturnValue(tempDir);

    // Mock process.exit to throw instead of exiting
    mockProcessExit = vi.spyOn(process, "exit").mockImplementation(() => {
      throw new Error("process.exit() was called");
    });

    // Reset all mocks
    vi.clearAllMocks();
  });

  afterEach(async () => {
    process.chdir(originalCwd);

    // Restore mocks
    vi.restoreAllMocks();

    // Clean up temp directory
    await rm(tempDir, { recursive: true, force: true });
  });

  describe("successful initialization", () => {
    it("should initialize project with auto-detected name", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: true, message: "Step 1 completed" },
        { success: true, message: "Step 2 completed" },
      ]);

      expect(() => initCommand()).not.toThrow();

      expect(performMultiStepInit).toHaveBeenCalledWith(
        expect.any(String), // project name
        tempDir
      );
      expect(success).toHaveBeenCalled();
    });

    it("should initialize project with provided name", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: true, message: "Step 1 completed" },
        { success: true, message: "Step 2 completed" },
      ]);

      expect(() => initCommand("my-project")).not.toThrow();

      expect(performMultiStepInit).toHaveBeenCalledWith("my-project", tempDir);
    });
  });

  describe("error conditions", () => {
    it("should exit if current directory is not a git repository", () => {
      vi.mocked(isGitRepository).mockReturnValue(false);

      expect(() => initCommand()).toThrow("process.exit() was called");

      expect(error).toHaveBeenCalledWith(
        "current directory is not a git repository"
      );
      expect(instruction).toHaveBeenCalled();
    });

    it("should show info message if project already initialized", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(true);

      expect(() => initCommand()).not.toThrow();

      expect(info).toHaveBeenCalledWith(
        expect.stringContaining("is already initialized")
      );
    });

    it("should handle partial initialization failure", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: true, message: "Config created" },
        { success: false, error: "Directory creation failed" },
      ]);

      expect(() => initCommand()).toThrow("process.exit() was called");

      expect(error).toHaveBeenCalledWith(
        expect.stringContaining("Initialization partially completed")
      );
    });

    it("should handle complete initialization failure", () => {
      vi.mocked(isGitRepository).mockReturnValue(true);
      vi.mocked(hasClaudectlConfig).mockReturnValue(false);
      vi.mocked(performMultiStepInit).mockReturnValue([
        { success: false, error: "Config creation failed" },
        { success: false, error: "Directory creation failed" },
      ]);

      expect(() => initCommand()).toThrow("process.exit() was called");

      expect(error).toHaveBeenCalledWith(
        expect.stringContaining("Initialization partially completed")
      );
    });
  });
});