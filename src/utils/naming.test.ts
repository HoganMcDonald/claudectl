import { describe, expect, it } from "vitest";
import { generateRandomName } from "./naming.js";

describe("naming utilities", () => {
  describe("generateRandomName", () => {
    it("should generate names in adjective-animal format", () => {
      const name = generateRandomName();

      expect(typeof name).toBe("string");
      expect(name).toMatch(/^[a-z]+-[a-z]+$/);

      const parts = name.split("-");
      expect(parts).toHaveLength(2);
      expect(parts[0].length).toBeGreaterThan(0);
      expect(parts[1].length).toBeGreaterThan(0);
    });

    it("should generate different names on multiple calls", () => {
      const names = new Set();

      // Generate 20 names and expect at least some variation
      for (let i = 0; i < 20; i++) {
        names.add(generateRandomName());
      }

      // With hundreds of possible combinations, we should get some variety
      expect(names.size).toBeGreaterThan(1);
    });

    it("should generate professional-sounding names", () => {
      // Test that generated names sound reasonable
      const name = generateRandomName();
      const [adjective, animal] = name.split("-");

      // Basic checks that these are actual words (not empty or weird)
      expect(adjective.length).toBeGreaterThan(2);
      expect(animal.length).toBeGreaterThan(2);
      expect(adjective).toMatch(/^[a-z]+$/);
      expect(animal).toMatch(/^[a-z]+$/);
    });
  });
});