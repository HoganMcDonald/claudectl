import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

// Mock tabtab
vi.mock("tabtab", () => ({
  default: {
    parseEnv: vi.fn(),
    log: vi.fn(),
    install: vi.fn(),
    uninstall: vi.fn(),
  },
}));

// Mock utils
vi.mock("./utils", () => ({
  hasClaudectlConfig: vi.fn(),
  loadProjectConfig: vi.fn(),
  getProjectWorktrees: vi.fn(),
  getWorktreeName: vi.fn(),
}));

// Import after mocking
const tabtab = await import("tabtab");
const {
  hasClaudectlConfig,
  loadProjectConfig,
  getProjectWorktrees,
  getWorktreeName,
} = await import("./utils");
const { handleCompletion, installCompletion, uninstallCompletion } =
  await import("./completion");

describe("completion", () => {
  let tempDir: string;
  let originalCwd: string;
  let originalEnv: NodeJS.ProcessEnv;

  beforeEach(async () => {
    tempDir = await mkdtemp(join(tmpdir(), "claudectl-test-"));
    originalCwd = process.cwd();
    originalEnv = { ...process.env };

    // Set test environment
    process.env.NODE_ENV = 'test';

    // Mock process.cwd to return our temp directory
    vi.spyOn(process, "cwd").mockReturnValue(tempDir);

    // Reset all mocks
    vi.clearAllMocks();
  });

  afterEach(async () => {
    process.chdir(originalCwd);
    process.env = originalEnv;
    
    // Restore mocks
    vi.restoreAllMocks();

    // Clean up temp directory
    await rm(tempDir, { recursive: true, force: true });
  });

  describe("handleCompletion", () => {
    it("should return early if not completing", async () => {
      vi.mocked(tabtab.default.parseEnv).mockReturnValue({
        complete: false,
        line: "",
        last: "",
      });

      await handleCompletion();

      expect(tabtab.default.log).not.toHaveBeenCalled();
    });

    it("should complete main commands for basic input", async () => {
      vi.mocked(tabtab.default.parseEnv).mockReturnValue({
        complete: true,
        line: "claudectl ",
        last: "",
      });

      await handleCompletion();

      expect(tabtab.default.log).toHaveBeenCalledWith([
        "init",
        "new", 
        "list",
        "rm",
        "attach",
        "tui"
      ]);
    });

    it("should complete session names for rm command", async () => {
      vi.mocked(tabtab.default.parseEnv).mockReturnValue({
        complete: true,
        line: "claudectl rm ",
        last: "",
      });

      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: "test-project" });
      vi.mocked(getProjectWorktrees).mockReturnValue([
        { path: "/test/brave-penguin", isMain: false } as any,
      ]);
      vi.mocked(getWorktreeName).mockReturnValue("brave-penguin");

      await handleCompletion();

      expect(tabtab.default.log).toHaveBeenCalledWith(["brave-penguin"]);
    });

    it("should complete flags for rm command after session name", async () => {
      vi.mocked(tabtab.default.parseEnv).mockReturnValue({
        complete: true,
        line: "claudectl rm session-name ",
        last: "",
      });

      await handleCompletion();

      expect(tabtab.default.log).toHaveBeenCalledWith(["--force", "-f"]);
    });

    it("should complete session names for attach command", async () => {
      vi.mocked(tabtab.default.parseEnv).mockReturnValue({
        complete: true,
        line: "claudectl attach ",
        last: "",
      });

      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockReturnValue({ name: "test-project" });
      vi.mocked(getProjectWorktrees).mockReturnValue([
        { path: "/test/brave-penguin", isMain: false } as any,
      ]);
      vi.mocked(getWorktreeName).mockReturnValue("brave-penguin");

      await handleCompletion();

      expect(tabtab.default.log).toHaveBeenCalledWith(["brave-penguin"]);
    });

    it("should filter completions based on partial input", async () => {
      vi.mocked(tabtab.default.parseEnv).mockReturnValue({
        complete: true,
        line: "claudectl in",
        last: "in",
      });

      await handleCompletion();

      expect(tabtab.default.log).toHaveBeenCalledWith(["init"]);
    });

    it("should handle errors gracefully when loading project config", async () => {
      vi.mocked(tabtab.default.parseEnv).mockReturnValue({
        complete: true,
        line: "claudectl rm ",
        last: "",
      });

      vi.mocked(hasClaudectlConfig).mockReturnValue(true);
      vi.mocked(loadProjectConfig).mockImplementation(() => {
        throw new Error("Config loading failed");
      });

      await handleCompletion();

      expect(tabtab.default.log).toHaveBeenCalledWith([]);
    });

    it("should handle no claudectl config", async () => {
      vi.mocked(tabtab.default.parseEnv).mockReturnValue({
        complete: true,
        line: "claudectl rm ",
        last: "",
      });

      vi.mocked(hasClaudectlConfig).mockReturnValue(false);

      await handleCompletion();

      expect(tabtab.default.log).toHaveBeenCalledWith([]);
    });
  });

  describe("installCompletion", () => {
    it("should install completion successfully", async () => {
      const mockConsoleLog = vi.spyOn(console, "log").mockImplementation();
      vi.mocked(tabtab.default.install).mockResolvedValue(undefined);

      await installCompletion();

      expect(tabtab.default.install).toHaveBeenCalledWith({
        name: "claudectl",
        completer: "claudectl",
      });
      expect(mockConsoleLog).toHaveBeenCalledWith(
        "Tab completion installed successfully"
      );

      mockConsoleLog.mockRestore();
    });

    it("should handle installation failure", async () => {
      const mockConsoleError = vi.spyOn(console, "error").mockImplementation();
      const mockProcessExit = vi.spyOn(process, "exit").mockImplementation(() => {
        throw new Error("process.exit() was called");
      });

      const error = new Error("Installation failed");
      vi.mocked(tabtab.default.install).mockRejectedValue(error);

      await expect(installCompletion()).rejects.toThrow(
        "process.exit() was called"
      );

      expect(mockConsoleError).toHaveBeenCalledWith(
        "Failed to install tab completion:",
        error
      );

      mockConsoleError.mockRestore();
      mockProcessExit.mockRestore();
    });

    it("should handle auto-install failure gracefully", async () => {
      const originalEnv = process.env.CLAUDECTL_AUTO_INSTALL;
      process.env.CLAUDECTL_AUTO_INSTALL = "true";

      const error = new Error("Installation failed");
      vi.mocked(tabtab.default.install).mockRejectedValue(error);

      await expect(installCompletion()).rejects.toThrow("Installation failed");

      process.env.CLAUDECTL_AUTO_INSTALL = originalEnv;
    });

    it("should not show messages during auto-install", async () => {
      const originalEnv = process.env.CLAUDECTL_AUTO_INSTALL;
      process.env.CLAUDECTL_AUTO_INSTALL = "true";

      const mockConsoleLog = vi.spyOn(console, "log").mockImplementation();
      vi.mocked(tabtab.default.install).mockResolvedValue(undefined);

      await installCompletion();

      expect(mockConsoleLog).not.toHaveBeenCalled();

      mockConsoleLog.mockRestore();
      process.env.CLAUDECTL_AUTO_INSTALL = originalEnv;
    });
  });

  describe("uninstallCompletion", () => {
    it("should uninstall completion successfully", async () => {
      const mockConsoleLog = vi.spyOn(console, "log").mockImplementation();
      vi.mocked(tabtab.default.uninstall).mockResolvedValue(undefined);

      await uninstallCompletion();

      expect(tabtab.default.uninstall).toHaveBeenCalledWith({
        name: "claudectl",
      });
      expect(mockConsoleLog).toHaveBeenCalledWith(
        "Tab completion uninstalled successfully"
      );

      mockConsoleLog.mockRestore();
    });

    it("should handle uninstallation failure", async () => {
      const mockConsoleError = vi.spyOn(console, "error").mockImplementation();
      const mockProcessExit = vi.spyOn(process, "exit").mockImplementation(() => {
        throw new Error("process.exit() was called");
      });

      const error = new Error("Uninstallation failed");
      vi.mocked(tabtab.default.uninstall).mockRejectedValue(error);

      await expect(uninstallCompletion()).rejects.toThrow(
        "process.exit() was called"
      );

      expect(mockConsoleError).toHaveBeenCalledWith(
        "Failed to uninstall tab completion:",
        error
      );

      mockConsoleError.mockRestore();
      mockProcessExit.mockRestore();
    });
  });
});