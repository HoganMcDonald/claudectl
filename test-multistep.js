// Test the multi-step initialization logic
console.log("=== Multi-Step Initialization Test ===\n");

// Simulate the step output format
const colors = {
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  cyan: '\x1b[36m',
  reset: '\x1b[0m',
  bold: '\x1b[1m',
  dim: '\x1b[2m',
};

function colorize(text, color) {
  return `${colors[color]}${text}${colors.reset}`;
}

// Simulate the multi-step initialization output
console.log(colorize("Initializing ClaudeCtl project \"my-project\":", "bold"));
console.log("");

// Step 1: Project configuration
console.log(`${colorize('[1/3]', 'cyan')} ${colorize('→', 'cyan')} Project configuration`);
console.log(`${colorize('✓', 'green')} ${colorize('Created .claudectl/config.json', 'green')}`);

// Step 2: Global directory structure  
console.log(`${colorize('[2/3]', 'cyan')} ${colorize('→', 'cyan')} Global directory structure`);
console.log(`${colorize('✓', 'green')} ${colorize('Created ~/.claudectl/projects/my-project', 'green')}`);

// Step 3: Git worktree
console.log(`${colorize('[3/3]', 'cyan')} ${colorize('→', 'cyan')} Git worktree "__main__"`);
console.log(`${colorize('✓', 'green')} ${colorize('Created Checked out main branch', 'green')}`);

console.log("");
console.log(`${colorize('✓', 'green')} ${colorize('Created ClaudeCtl project "my-project" initialized successfully', 'green')}`);
console.log(`${colorize('ℹ', 'blue')} ${colorize('You can now use ClaudeCtl commands in this project', 'blue')}`);

console.log("\n=== Test Complete ===");