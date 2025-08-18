import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { ZodError } from "zod";
import type { ProjectConfig } from "../types";
import {
  createProjectConfig,
  loadProjectConfig,
  updateProjectConfig,
} from "./config";

describe("config utilities", () => {
  let tempDir: string;

  beforeEach(async () => {
    tempDir = await mkdtemp(join(tmpdir(), "claudectl-test-"));
  });

  afterEach(async () => {
    if (tempDir) {
      await rm(tempDir, { recursive: true, force: true });
    }
  });

  describe("createProjectConfig", () => {
    it("should create .claudectl directory and config.json file", async () => {
      const config: ProjectConfig = { name: "test-project" };

      createProjectConfig(tempDir, config);

      // Check that config file was created
      const configPath = join(tempDir, ".claudectl", "config.json");
      const configContent = await readFile(configPath, "utf-8");
      const parsedConfig = JSON.parse(configContent);

      expect(parsedConfig).toEqual(config);
    });

    it("should validate config with Zod schema", () => {
      const invalidConfig = { name: "" } as ProjectConfig; // Empty name should fail validation

      expect(() => createProjectConfig(tempDir, invalidConfig)).toThrow(
        ZodError
      );
    });

    it("should format JSON with proper indentation", async () => {
      const config: ProjectConfig = { name: "test-project" };

      createProjectConfig(tempDir, config);

      const configPath = join(tempDir, ".claudectl", "config.json");
      const configContent = await readFile(configPath, "utf-8");

      // Check that JSON is properly formatted with 2-space indentation
      expect(configContent).toBe('{\n  "name": "test-project"\n}');
    });

    it("should not overwrite existing .claudectl directory", async () => {
      // Create .claudectl directory with some content first
      const claudectlDir = join(tempDir, ".claudectl");
      await mkdir(claudectlDir, { recursive: true });
      const existingFile = join(claudectlDir, "existing-file.txt");
      await writeFile(existingFile, "existing content");

      const config: ProjectConfig = { name: "test-project" };
      createProjectConfig(tempDir, config);

      // Check that existing file is still there
      const existingContent = await readFile(existingFile, "utf-8");
      expect(existingContent).toBe("existing content");

      // And new config file is also there
      const configPath = join(claudectlDir, "config.json");
      const configContent = await readFile(configPath, "utf-8");
      const parsedConfig = JSON.parse(configContent);
      expect(parsedConfig).toEqual(config);
    });
  });

  describe("loadProjectConfig", () => {
    it("should load and validate existing config file", async () => {
      const config: ProjectConfig = { name: "test-project" };
      createProjectConfig(tempDir, config);

      const loadedConfig = loadProjectConfig(tempDir);
      expect(loadedConfig).toEqual(config);
    });

    it("should throw error if config file does not exist", () => {
      expect(() => loadProjectConfig(tempDir)).toThrow(
        "No claudectl config found. Run 'claudectl init' first."
      );
    });

    it("should throw error for invalid JSON", async () => {
      const claudectlDir = join(tempDir, ".claudectl");
      const configPath = join(claudectlDir, "config.json");

      // Create directory and write invalid JSON
      await mkdir(claudectlDir, { recursive: true });
      await writeFile(configPath, "invalid json content");

      expect(() => loadProjectConfig(tempDir)).toThrow();
    });

    it("should validate config with Zod schema and throw for invalid data", async () => {
      const claudectlDir = join(tempDir, ".claudectl");
      const configPath = join(claudectlDir, "config.json");

      // Create directory and write invalid config
      await mkdir(claudectlDir, { recursive: true });
      await writeFile(configPath, JSON.stringify({ name: "" })); // Empty name should fail

      expect(() => loadProjectConfig(tempDir)).toThrow(ZodError);
    });

    it("should use current working directory when no path provided", () => {
      // Mock process.cwd to return our temp directory (which has no config)
      const originalCwd = process.cwd;
      vi.spyOn(process, "cwd").mockReturnValue(tempDir);

      expect(() => loadProjectConfig()).toThrow(
        "No claudectl config found. Run 'claudectl init' first."
      );

      // Restore original cwd
      process.cwd = originalCwd;
    });
  });

  describe("updateProjectConfig", () => {
    it("should update existing config with new values", async () => {
      const originalConfig: ProjectConfig = { name: "original-name" };
      createProjectConfig(tempDir, originalConfig);

      const updates = { name: "updated-name" };
      updateProjectConfig(tempDir, updates);

      const updatedConfig = loadProjectConfig(tempDir);
      expect(updatedConfig.name).toBe("updated-name");
    });

    it("should preserve existing properties when doing partial updates", async () => {
      const originalConfig: ProjectConfig = { name: "original-name" };
      createProjectConfig(tempDir, originalConfig);

      // In the future when we have more properties, this test will be more meaningful
      // For now, we're just testing that the merge logic works
      const updates = { name: "updated-name" };
      updateProjectConfig(tempDir, updates);

      const updatedConfig = loadProjectConfig(tempDir);
      expect(updatedConfig).toEqual({ name: "updated-name" });
    });

    it("should validate merged config with Zod schema", async () => {
      const originalConfig: ProjectConfig = { name: "original-name" };
      createProjectConfig(tempDir, originalConfig);

      const invalidUpdates = { name: "" }; // Empty name should fail validation

      expect(() => updateProjectConfig(tempDir, invalidUpdates)).toThrow(
        ZodError
      );
    });

    it("should throw error if original config does not exist", () => {
      const updates = { name: "new-name" };

      expect(() => updateProjectConfig(tempDir, updates)).toThrow(
        "No claudectl config found. Run 'claudectl init' first."
      );
    });
  });
});
