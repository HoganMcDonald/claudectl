/**
 * Name generation utilities for ClaudeCtl.
 *
 * This module provides functions for generating human-friendly names using
 * adjective-animal patterns for worktrees and other resources.
 */

/**
 * List of positive adjectives for name generation.
 * Focused on qualities that are positive, memorable, and professional.
 */
const ADJECTIVES = [
  // Positive traits
  "brave", "clever", "swift", "bright", "calm", "eager", "wise", "bold",
  "gentle", "quick", "smart", "strong", "keen", "noble", "agile", "alert",
  
  // Colors
  "crimson", "golden", "silver", "azure", "emerald", "violet", "coral", "amber",
  "indigo", "scarlet", "turquoise", "bronze", "platinum", "ruby", "sapphire",
  
  // Natural qualities
  "flowing", "radiant", "shining", "sparkling", "glowing", "blazing", "crystal",
  "misty", "frosty", "sunny", "stormy", "windy", "cloudy", "starry", "lunar",
  
  // Size/movement
  "rapid", "steady", "fluid", "nimble", "sleek", "graceful", "mighty", "tiny",
  "giant", "swift", "silent", "roaring", "whispering", "dancing", "leaping",
  
  // Personality
  "curious", "playful", "serene", "vibrant", "cheerful", "focused", "patient",
  "creative", "witty", "loyal", "honest", "friendly", "confident", "humble"
] as const;

/**
 * List of animals for name generation.
 * Includes a diverse mix of animals that are recognizable and memorable.
 */
const ANIMALS = [
  // Mammals
  "fox", "wolf", "bear", "lion", "tiger", "leopard", "cheetah", "panda",
  "elephant", "rhino", "hippo", "giraffe", "zebra", "deer", "moose", "elk",
  "rabbit", "hare", "squirrel", "beaver", "otter", "seal", "whale", "dolphin",
  "cat", "dog", "horse", "cow", "sheep", "goat", "pig", "llama", "alpaca",
  
  // Birds
  "eagle", "hawk", "falcon", "owl", "raven", "crow", "robin", "sparrow",
  "cardinal", "hummingbird", "penguin", "flamingo", "swan", "duck", "goose",
  "pelican", "crane", "heron", "kingfisher", "woodpecker", "parrot", "peacock",
  
  // Reptiles & Amphibians
  "turtle", "tortoise", "lizard", "gecko", "iguana", "snake", "python",
  "frog", "toad", "salamander", "chameleon", "dragon", "cobra",
  
  // Fish & Marine
  "shark", "ray", "tuna", "salmon", "trout", "bass", "cod", "mackerel",
  "octopus", "squid", "jellyfish", "starfish", "seahorse", "clownfish",
  
  // Insects & Others
  "butterfly", "bee", "ant", "spider", "beetle", "dragonfly", "firefly",
  "cricket", "grasshopper", "mantis", "scorpion", "crab", "lobster"
] as const;

/**
 * Generates a random adjective-animal name pattern.
 *
 * @returns A string in the format "adjective-animal" (e.g., "brave-penguin").
 *
 * @example
 * ```typescript
 * const name1 = generateRandomName(); // "clever-fox"
 * const name2 = generateRandomName(); // "golden-eagle"
 * const name3 = generateRandomName(); // "swift-dolphin"
 * ```
 */
export function generateRandomName(): string {
  const adjective = ADJECTIVES[Math.floor(Math.random() * ADJECTIVES.length)];
  const animal = ANIMALS[Math.floor(Math.random() * ANIMALS.length)];
  return `${adjective}-${animal}`;
}

/**
 * Generates multiple unique random names.
 *
 * @param count - The number of unique names to generate.
 * @param maxAttempts - Maximum attempts to generate unique names (defaults to count * 10).
 * @returns Array of unique adjective-animal names.
 * @throws {Error} When unable to generate enough unique names.
 *
 * @example
 * ```typescript
 * const names = generateUniqueNames(3);
 * // ["brave-penguin", "swift-fox", "golden-eagle"]
 * ```
 */
export function generateUniqueNames(count: number, maxAttempts: number = count * 10): string[] {
  const names = new Set<string>();
  let attempts = 0;

  while (names.size < count && attempts < maxAttempts) {
    names.add(generateRandomName());
    attempts++;
  }

  if (names.size < count) {
    throw new Error(`Unable to generate ${count} unique names after ${maxAttempts} attempts`);
  }

  return Array.from(names);
}

/**
 * Validates that a name follows the adjective-animal pattern.
 *
 * @param name - The name to validate.
 * @returns True if the name matches the expected pattern.
 *
 * @example
 * ```typescript
 * isValidAdjectiveAnimalName("brave-penguin"); // true
 * isValidAdjectiveAnimalName("invalid_name"); // false
 * isValidAdjectiveAnimalName("too-many-parts"); // false
 * ```
 */
export function isValidAdjectiveAnimalName(name: string): boolean {
  const parts = name.split('-');
  if (parts.length !== 2) {
    return false;
  }

  const [adjective, animal] = parts;
  return (
    ADJECTIVES.includes(adjective as typeof ADJECTIVES[number]) &&
    ANIMALS.includes(animal as typeof ANIMALS[number])
  );
}

/**
 * Gets the total number of possible adjective-animal combinations.
 *
 * @returns The total number of unique combinations possible.
 *
 * @example
 * ```typescript
 * const total = getTotalCombinations(); // 4680 (if 60 adjectives * 78 animals)
 * ```
 */
export function getTotalCombinations(): number {
  return ADJECTIVES.length * ANIMALS.length;
}

/**
 * Generates a name with a specific adjective.
 *
 * @param adjective - The specific adjective to use.
 * @returns A name with the specified adjective and random animal.
 * @throws {Error} When the adjective is not in the allowed list.
 *
 * @example
 * ```typescript
 * const name = generateNameWithAdjective("brave"); // "brave-penguin"
 * ```
 */
export function generateNameWithAdjective(adjective: string): string {
  if (!ADJECTIVES.includes(adjective as typeof ADJECTIVES[number])) {
    throw new Error(`Invalid adjective: ${adjective}`);
  }

  const animal = ANIMALS[Math.floor(Math.random() * ANIMALS.length)];
  return `${adjective}-${animal}`;
}

/**
 * Generates a name with a specific animal.
 *
 * @param animal - The specific animal to use.
 * @returns A name with random adjective and the specified animal.
 * @throws {Error} When the animal is not in the allowed list.
 *
 * @example
 * ```typescript
 * const name = generateNameWithAnimal("fox"); // "clever-fox"
 * ```
 */
export function generateNameWithAnimal(animal: string): string {
  if (!ANIMALS.includes(animal as typeof ANIMALS[number])) {
    throw new Error(`Invalid animal: ${animal}`);
  }

  const adjective = ADJECTIVES[Math.floor(Math.random() * ADJECTIVES.length)];
  return `${adjective}-${animal}`;
}