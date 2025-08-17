#!/usr/bin/env node

/**
 * Pre-uninstall script for claudectl
 * Automatically removes tab completion when uninstalled globally
 */

const { spawn } = require('child_process');

/**
 * Check if this is a global uninstallation
 */
function isGlobalUninstall() {
  // Check if we're being uninstalled globally
  if (process.env.npm_config_global === 'true') {
    return true;
  }
  
  // Alternative check: see if we're in a global node_modules
  const installPath = process.cwd();
  return installPath.includes('node_modules') && 
         (installPath.includes('/usr/local/lib/node_modules') || 
          installPath.includes('/usr/lib/node_modules') ||
          installPath.includes('/.npm-global/lib/node_modules') ||
          installPath.includes('/node_modules/.pnpm-global') ||
          installPath.includes('AppData/Roaming/npm/node_modules')); // Windows
}

/**
 * Uninstall tab completion
 */
async function uninstallCompletion() {
  return new Promise((resolve) => {
    console.log('🧹 Removing claudectl tab completion...');
    
    const child = spawn('claudectl', ['uninstall-completion'], {
      stdio: 'inherit',
      env: {
        ...process.env,
        CLAUDECTL_AUTO_UNINSTALL: 'true' // Flag to indicate this is automatic
      }
    });
    
    child.on('close', (code) => {
      if (code === 0) {
        console.log('✅ Tab completion removed.');
      } else {
        console.log('⚠️  Could not remove tab completion automatically. You may need to remove it manually.');
      }
      resolve();
    });
    
    child.on('error', (error) => {
      console.log('⚠️  Could not remove tab completion:', error.message);
      resolve(); // Don't fail the whole uninstallation
    });
  });
}

/**
 * Main function
 */
async function main() {
  try {
    // Only uninstall completion for global uninstalls
    if (!isGlobalUninstall()) {
      return;
    }
    
    console.log('🌍 Uninstalling claudectl globally...');
    
    // Remove completion
    await uninstallCompletion();
    
  } catch (error) {
    console.log('⚠️  Pre-uninstall cleanup encountered an issue:', error.message);
  }
}

// Only run if this script is being executed directly
if (require.main === module) {
  main();
}

module.exports = { isGlobalUninstall, uninstallCompletion };