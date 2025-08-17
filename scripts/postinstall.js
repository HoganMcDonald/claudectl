#!/usr/bin/env node

/**
 * Post-install script for claudectl
 * Automatically installs tab completion when installed globally
 */

const { spawn } = require('child_process');
const path = require('path');

/**
 * Check if this is a global installation
 */
function isGlobalInstall() {
  // Check if we're being installed globally by looking at the npm_config_global env var
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
 * Check if completion is already installed
 */
async function isCompletionInstalled() {
  return new Promise((resolve) => {
    // Try to run claudectl with a completion environment to see if it's set up
    const env = {
      ...process.env,
      COMP_LINE: 'claudectl ',
      COMP_POINT: '9',
    };
    
    const child = spawn('claudectl', [], { 
      env,
      stdio: 'pipe',
      timeout: 2000 
    });
    
    child.on('close', (code) => {
      // If completion is working, it should exit cleanly
      resolve(code === 0);
    });
    
    child.on('error', () => {
      resolve(false);
    });
  });
}

/**
 * Install tab completion
 */
async function installCompletion() {
  return new Promise((resolve, reject) => {
    console.log('🔧 Setting up tab completion for claudectl...');
    
    const child = spawn('claudectl', ['install-completion'], {
      stdio: 'inherit',
      env: {
        ...process.env,
        CLAUDECTL_AUTO_INSTALL: 'true' // Flag to indicate this is automatic
      }
    });
    
    child.on('close', (code) => {
      if (code === 0) {
        console.log('✅ Tab completion installed! Restart your shell to enable it.');
        resolve();
      } else {
        console.log('⚠️  Tab completion installation failed. You can install it manually with: claudectl install-completion');
        resolve(); // Don't fail the whole installation
      }
    });
    
    child.on('error', (error) => {
      console.log('⚠️  Could not install tab completion automatically:', error.message);
      console.log('💡 You can install it manually with: claudectl install-completion');
      resolve(); // Don't fail the whole installation
    });
  });
}

/**
 * Main function
 */
async function main() {
  try {
    // Only install completion for global installs
    if (!isGlobalInstall()) {
      console.log('📦 claudectl installed locally. Run "npm install -g claudectl" for global installation with tab completion.');
      return;
    }
    
    console.log('🌍 claudectl installed globally!');
    
    // Wait a moment for the binary to be available
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Check if completion is already installed
    const alreadyInstalled = await isCompletionInstalled();
    if (alreadyInstalled) {
      console.log('✅ Tab completion already configured.');
      return;
    }
    
    // Install completion
    await installCompletion();
    
  } catch (error) {
    console.log('⚠️  Post-install setup encountered an issue:', error.message);
    console.log('💡 You can manually install tab completion with: claudectl install-completion');
  }
}

// Only run if this script is being executed directly
if (require.main === module) {
  main();
}

module.exports = { isGlobalInstall, installCompletion };