# Deployment Guide

This document explains the CI/CD setup for claudectl.

## Workflows

### CI Workflow (`ci.yml`)
Runs on every push and pull request to main:
- **Test**: Runs `cargo test`, `cargo clippy`, and `cargo fmt --check`
- **Build**: Builds on Ubuntu, Windows, and macOS  
- **Test NPM**: Tests npm package scripts and functionality
- **Completions**: Verifies shell completion generation works

### Release Workflow (`release.yml`) 
Runs when code merges to main with version changes:
- **Version Detection**: Automatically detects version bumps in `package.json` or `Cargo.toml`
- **Multi-platform Build**: Builds binaries for:
  - Linux (x64, ARM64)
  - macOS (x64, ARM64) 
  - Windows (x64)
- **Completions**: Generates shell completions for all supported shells
- **GitHub Release**: Creates release with binaries and completions
- **NPM Publish**: Publishes package to npm with platform-specific binaries

## Release Process

### Automatic Release
1. Update version in `package.json` (and optionally `Cargo.toml`)
2. Commit and push to main
3. Workflow automatically detects version change
4. Builds, tests, and releases

### Manual Release
Use GitHub's "Run workflow" button with force-release option.

## Setup Requirements

### Repository Secrets
Add these secrets in GitHub Settings > Secrets and variables > Actions:

1. **`NPM_TOKEN`**: npm access token for publishing
   - Go to npmjs.com > Access Tokens > Generate New Token
   - Choose "Automation" type
   - Add token to repository secrets

### npm Package Structure
The npm package includes:
- `npm/run.js`: Platform-detection wrapper script
- `npm/install.js`: Auto-installs shell completions  
- `npm/uninstall.js`: Cleans up completions
- `npm/bin/`: Platform-specific binaries (added during CI)
- `completions/`: Shell completion files
- `target/release/claudectl`: Default binary for npm scripts

### Platform Binary Selection
The `npm/run.js` script automatically selects the correct binary:
- Checks `npm/bin/` for platform-specific binary first
- Falls back to development build in `target/release/`  
- Finally falls back to system PATH

## Testing Locally

### Test CI workflow:
```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
node npm/install.js
node npm/run.js --help
```

### Test release build:
```bash
cargo build --release --target x86_64-unknown-linux-gnu
./target/x86_64-unknown-linux-gnu/release/claudectl completions zsh > /tmp/test.zsh
```

### Test npm package:
```bash
npm pack  # Creates tarball
npm install -g ./claudectl-*.tgz
claudectl --help
npm uninstall -g claudectl
```

## Version Management

- **Primary source**: `package.json` version field
- **Secondary**: `Cargo.toml` version (should match)
- **Tagging**: Workflow creates `vX.Y.Z` tags automatically
- **npm versioning**: Use `npm version patch/minor/major` to bump both files

## Troubleshooting

### Release not triggering
- Check version was actually changed in git diff
- Verify commit contains `package.json` or `Cargo.toml` changes
- Check workflow run logs for version detection

### npm publish failing  
- Verify `NPM_TOKEN` secret is set correctly
- Check npm package name availability
- Ensure version isn't already published

### Binary compatibility issues
- Verify target platform support in build matrix
- Check cross-compilation setup for ARM64 Linux
- Test platform detection logic in `npm/run.js`