/**
 * Simple name generation for ClaudeCtl worktrees.
 */

const ADJECTIVES = [
  "brave", "clever", "swift", "bright", "calm", "wise", "bold", "gentle", 
  "quick", "smart", "strong", "keen", "noble", "agile", "alert", "golden",
  "silver", "crimson", "azure", "emerald", "graceful", "mighty", "serene"
] as const;

const ANIMALS = [
  "fox", "wolf", "bear", "lion", "tiger", "panda", "eagle", "hawk", 
  "owl", "raven", "dolphin", "whale", "shark", "penguin", "seahorse",
  "butterfly", "dragon", "phoenix", "falcon", "cobra", "turtle", "rabbit"
] as const;

/**
 * Generates a random adjective-animal name (e.g., "brave-penguin").
 */
export function generateRandomName(): string {
  const adjective = ADJECTIVES[Math.floor(Math.random() * ADJECTIVES.length)];
  const animal = ANIMALS[Math.floor(Math.random() * ANIMALS.length)];
  return `${adjective}-${animal}`;
}