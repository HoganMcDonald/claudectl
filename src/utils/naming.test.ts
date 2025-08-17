import { describe, it, expect } from 'vitest';
import {
  generateRandomName,
  generateUniqueNames,
  isValidAdjectiveAnimalName,
  getTotalCombinations,
  generateNameWithAdjective,
  generateNameWithAnimal,
} from './naming';

describe('naming utilities', () => {
  describe('generateRandomName', () => {
    it('should generate names in adjective-animal format', () => {
      const name = generateRandomName();
      
      expect(typeof name).toBe('string');
      expect(name).toMatch(/^[a-z]+-[a-z]+$/);
      
      const parts = name.split('-');
      expect(parts).toHaveLength(2);
      expect(parts[0].length).toBeGreaterThan(0);
      expect(parts[1].length).toBeGreaterThan(0);
    });

    it('should generate different names on multiple calls', () => {
      const names = new Set();
      
      // Generate 20 names and expect at least some variation
      for (let i = 0; i < 20; i++) {
        names.add(generateRandomName());
      }
      
      // With thousands of possible combinations, we should get some variety
      expect(names.size).toBeGreaterThan(1);
    });

    it('should only generate valid adjective-animal combinations', () => {
      for (let i = 0; i < 10; i++) {
        const name = generateRandomName();
        expect(isValidAdjectiveAnimalName(name)).toBe(true);
      }
    });
  });

  describe('generateUniqueNames', () => {
    it('should generate the requested number of unique names', () => {
      const names = generateUniqueNames(5);
      
      expect(names).toHaveLength(5);
      expect(new Set(names).size).toBe(5); // All should be unique
      
      names.forEach(name => {
        expect(isValidAdjectiveAnimalName(name)).toBe(true);
      });
    });

    it('should handle edge case of generating 1 name', () => {
      const names = generateUniqueNames(1);
      
      expect(names).toHaveLength(1);
      expect(isValidAdjectiveAnimalName(names[0])).toBe(true);
    });

    it('should throw error when unable to generate enough unique names', () => {
      const totalCombinations = getTotalCombinations();
      
      expect(() => generateUniqueNames(totalCombinations + 1)).toThrow(
        `Unable to generate ${totalCombinations + 1} unique names`
      );
    });

    it('should respect maxAttempts parameter', () => {
      expect(() => generateUniqueNames(10, 5)).toThrow(
        'Unable to generate 10 unique names after 5 attempts'
      );
    });
  });

  describe('isValidAdjectiveAnimalName', () => {
    it('should return true for valid adjective-animal names', () => {
      // Test some known valid combinations
      expect(isValidAdjectiveAnimalName('brave-penguin')).toBe(true);
      expect(isValidAdjectiveAnimalName('swift-fox')).toBe(true);
      expect(isValidAdjectiveAnimalName('golden-eagle')).toBe(true);
    });

    it('should return false for invalid formats', () => {
      expect(isValidAdjectiveAnimalName('invalid')).toBe(false);
      expect(isValidAdjectiveAnimalName('too-many-parts')).toBe(false);
      expect(isValidAdjectiveAnimalName('no_dash')).toBe(false);
      expect(isValidAdjectiveAnimalName('')).toBe(false);
      expect(isValidAdjectiveAnimalName('-')).toBe(false);
      expect(isValidAdjectiveAnimalName('adjective-')).toBe(false);
      expect(isValidAdjectiveAnimalName('-animal')).toBe(false);
    });

    it('should return false for unknown adjectives or animals', () => {
      expect(isValidAdjectiveAnimalName('unknown-penguin')).toBe(false);
      expect(isValidAdjectiveAnimalName('brave-unicorn')).toBe(false);
      expect(isValidAdjectiveAnimalName('unknown-unicorn')).toBe(false);
    });
  });

  describe('getTotalCombinations', () => {
    it('should return a positive number', () => {
      const total = getTotalCombinations();
      
      expect(typeof total).toBe('number');
      expect(total).toBeGreaterThan(0);
    });

    it('should be consistent across calls', () => {
      const total1 = getTotalCombinations();
      const total2 = getTotalCombinations();
      
      expect(total1).toBe(total2);
    });

    it('should be a reasonable number for our word lists', () => {
      const total = getTotalCombinations();
      
      // We expect thousands of combinations (60+ adjectives × 80+ animals)
      expect(total).toBeGreaterThan(1000);
      expect(total).toBeLessThan(100000); // Sanity check
    });
  });

  describe('generateNameWithAdjective', () => {
    it('should generate name with specified adjective', () => {
      const name = generateNameWithAdjective('brave');
      
      expect(name).toMatch(/^brave-[a-z]+$/);
      expect(isValidAdjectiveAnimalName(name)).toBe(true);
    });

    it('should work with different adjectives', () => {
      const name1 = generateNameWithAdjective('swift');
      const name2 = generateNameWithAdjective('golden');
      
      expect(name1).toMatch(/^swift-[a-z]+$/);
      expect(name2).toMatch(/^golden-[a-z]+$/);
      expect(isValidAdjectiveAnimalName(name1)).toBe(true);
      expect(isValidAdjectiveAnimalName(name2)).toBe(true);
    });

    it('should throw error for invalid adjective', () => {
      expect(() => generateNameWithAdjective('invalid')).toThrow('Invalid adjective: invalid');
      expect(() => generateNameWithAdjective('')).toThrow('Invalid adjective: ');
    });

    it('should generate different animals with same adjective', () => {
      const names = new Set();
      
      // Generate multiple names with same adjective
      for (let i = 0; i < 10; i++) {
        names.add(generateNameWithAdjective('brave'));
      }
      
      // Should get some variety in animals
      expect(names.size).toBeGreaterThan(1);
    });
  });

  describe('generateNameWithAnimal', () => {
    it('should generate name with specified animal', () => {
      const name = generateNameWithAnimal('penguin');
      
      expect(name).toMatch(/^[a-z]+-penguin$/);
      expect(isValidAdjectiveAnimalName(name)).toBe(true);
    });

    it('should work with different animals', () => {
      const name1 = generateNameWithAnimal('fox');
      const name2 = generateNameWithAnimal('eagle');
      
      expect(name1).toMatch(/^[a-z]+-fox$/);
      expect(name2).toMatch(/^[a-z]+-eagle$/);
      expect(isValidAdjectiveAnimalName(name1)).toBe(true);
      expect(isValidAdjectiveAnimalName(name2)).toBe(true);
    });

    it('should throw error for invalid animal', () => {
      expect(() => generateNameWithAnimal('unicorn')).toThrow('Invalid animal: unicorn');
      expect(() => generateNameWithAnimal('')).toThrow('Invalid animal: ');
    });

    it('should generate different adjectives with same animal', () => {
      const names = new Set();
      
      // Generate multiple names with same animal
      for (let i = 0; i < 10; i++) {
        names.add(generateNameWithAnimal('penguin'));
      }
      
      // Should get some variety in adjectives
      expect(names.size).toBeGreaterThan(1);
    });
  });

  describe('word list quality', () => {
    it('should have reasonable word list sizes', () => {
      const total = getTotalCombinations();
      
      // Reverse calculate approximate list sizes
      const sqrtApprox = Math.sqrt(total);
      
      // Both lists should be reasonably sized (20+ items each)
      expect(sqrtApprox).toBeGreaterThan(20);
    });

    it('should generate professional-sounding names', () => {
      for (let i = 0; i < 5; i++) {
        const name = generateRandomName();
        
        // Names should be lowercase and use only letters and hyphens
        expect(name).toMatch(/^[a-z]+-[a-z]+$/);
        
        // Should not contain any inappropriate patterns
        expect(name).not.toMatch(/\d/); // No numbers
        expect(name).not.toMatch(/[A-Z]/); // No uppercase
        expect(name).not.toMatch(/[^a-z-]/); // Only lowercase letters and hyphens
      }
    });
  });
});