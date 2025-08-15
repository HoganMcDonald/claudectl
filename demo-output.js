// Demo of enhanced output formatting
console.log("=== Enhanced CLI Output Demo ===\n");

// Simulate the enhanced functions for demonstration
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

// Error with red color and icon
console.log(`${colorize('✗', 'red')} ${colorize('error: current directory is not a git repository', 'red')}`);

// Warning with yellow color and icon  
console.log(`${colorize('⚠', 'yellow')} ${colorize('warning: this action cannot be undone', 'yellow')}`);

// Info with blue color and icon
console.log(`${colorize('ℹ', 'blue')} ${colorize('Project "my-project" is already initialized', 'blue')}`);

// Success with green color and icon
console.log(`${colorize('✓', 'green')} ${colorize('Created ClaudeCtl project "my-project"', 'green')}`);
console.log(`${colorize('✓', 'green')} ${colorize('Created .claudectl/config.json', 'green')}`);

console.log("\n" + colorize("Additional formatting examples:", "bold"));

// Step indicators
console.log(`${colorize('[1/3]', 'cyan')} ${colorize('→', 'cyan')} Initializing project`);
console.log(`${colorize('[2/3]', 'cyan')} ${colorize('→', 'cyan')} Creating configuration`);

// Progress bar
const bar = colorize('█'.repeat(6), 'green') + colorize('░'.repeat(4), 'dim');
console.log(`Installing dependencies [${bar}] 60%`);

// Table header
console.log(`\n${colorize('Name'.padEnd(15), 'bold')}${colorize('Status'.padEnd(10), 'bold')}${colorize('Path', 'bold')}`);
console.log(colorize('─'.repeat(15) + '  ' + '─'.repeat(10) + '  ' + '─'.repeat(20), 'dim'));
console.log('project-1'.padEnd(15) + '  ' + 'active'.padEnd(10) + '  ' + '/path/to/project');

console.log("\nDemo complete!");