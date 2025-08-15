// Simple test to verify our output functions work
const fs = require('fs');

// Simulate our output functions
function error(message, details) {
  console.error(`error: ${message}`);
  if (details) {
    console.error(`       ${details}`);
  }
}

function success(message) {
  console.log(`     Created ${message}`);
}

function info(message) {
  console.log(`  ${message}`);
}

function blank() {
  console.log("");
}

function codeBlock(lines) {
  const codeLines = Array.isArray(lines) ? lines : [lines];
  codeLines.forEach(line => {
    console.log(`    ${line}`);
  });
}

function instruction(title, commands) {
  console.log(title);
  blank();
  codeBlock(commands);
  blank();
}

// Test the functions
console.log("Testing output helpers:");
console.log("======================");

error("current directory is not a git repository");
instruction(
  "ClaudeCtl uses git worktrees for managing code contexts. Please initialize a git repository first:",
  [
    "git init",
    "git add .",
    'git commit -m "Initial commit"'
  ]
);

info('Project "test-project" is already initialized');

success('ClaudeCtl project "test-project"');
success(".claudectl/config.json");