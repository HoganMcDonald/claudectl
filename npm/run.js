#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

function getPlatformBinaryName() {
  const platform = process.platform;
  const arch = process.arch;
  
  // Map platform/arch combinations to binary names
  const binaryMap = {
    'linux-x64': 'claudectl-linux-x64',
    'linux-arm64': 'claudectl-linux-arm64', 
    'darwin-x64': 'claudectl-macos-x64',
    'darwin-arm64': 'claudectl-macos-arm64',
    'win32-x64': 'claudectl-windows-x64.exe',
  };
  
  const key = `${platform}-${arch}`;
  return binaryMap[key] || null;
}

function getBinaryPath() {
  const platform = process.platform;
  
  // Try platform-specific binary first (from npm package)
  const platformBinary = getPlatformBinaryName();
  if (platformBinary) {
    const platformPath = path.join(__dirname, 'bin', platformBinary);
    if (fs.existsSync(platformPath)) {
      return platformPath;
    }
  }
  
  // Try development build
  const binaryName = platform === 'win32' ? 'claudectl.exe' : 'claudectl';
  const devPath = path.join(__dirname, '..', 'target', 'release', binaryName);
  if (fs.existsSync(devPath)) {
    return devPath;
  }
  
  // Fallback to system PATH
  return 'claudectl';
}

function run() {
  const binaryPath = getBinaryPath();
  const args = process.argv.slice(2);
  
  const child = spawn(binaryPath, args, {
    stdio: 'inherit',
    windowsHide: false,
  });
  
  child.on('error', (error) => {
    if (error.code === 'ENOENT') {
      console.error('claudectl binary not found. Please ensure the package was installed correctly.');
      process.exit(1);
    } else {
      console.error('Error running claudectl:', error.message);
      process.exit(1);
    }
  });
  
  child.on('exit', (code) => {
    process.exit(code || 0);
  });
}

if (require.main === module) {
  run();
}

module.exports = { getBinaryPath, run };