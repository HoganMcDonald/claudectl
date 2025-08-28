# ClaudeCtl

[![CI](https://github.com/your-org/claudectl/workflows/CI/badge.svg)](https://github.com/your-org/claudectl/actions)

A CLI tool for orchestrating Claude Code agents through git worktrees.

## Commands

### `claudectl init`

Initialize the current project for claudectl. Must be run in a git repository with Claude Code installed.

### `claudectl task <task-name>`

Create a new git worktree for the specified task.

**Arguments:**
- `<task-name>`: Name of the task/branch (e.g., `feat/new-feature`)

### `claudectl list`

List all active task worktrees with their status.

### `claudectl rm <task-name>`

Remove a task worktree and clean up associated files.

**Arguments:**
- `<task-name>`: Name of the task to remove

### `claudectl completions [shell]`

Generate or manage shell completions.

**Options:**
- `--verify`: Check if completions are installed and working
- `--install`: Install completions automatically

**Arguments:**
- `[shell]`: Target shell (bash, zsh, fish, powershell, elvish)

### `claudectl repair`

Repair shell completions and fix common configuration issues.

**Options:**
- `--force`: Force repair even if completions appear working

### Global Options

- `--debug`: Enable debug logging output

## Installation

### Via npm (Recommended)

```bash
npm install -g claudectl
```

Shell completions are installed automatically. If they don't work:

```bash
# Verify installation
claudectl completions --verify

# Repair if needed
claudectl repair
```

### Manual Installation

1. Download the binary from [releases](https://github.com/your-org/claudectl/releases)
2. Install completions manually:

```bash
# Generate completions for your shell
claudectl completions zsh > ~/.zsh_completion.d/_claudectl

# Add to your shell config
echo 'fpath+=~/.zsh_completion.d' >> ~/.zshrc
echo 'autoload -U compinit && compinit' >> ~/.zshrc
```

## Shell Completions

Claudectl includes smart shell completions that:

- Complete command names and options
- Dynamically complete task names for `claudectl rm`
- Work across bash, zsh, fish, PowerShell, and elvish

### Troubleshooting Completions

If completions aren't working:

1. **Check installation**: `claudectl completions --verify`
2. **Repair automatically**: `claudectl repair`
3. **Manual repair**: `npm run setup`
4. **Reinstall completely**: `npm uninstall -g claudectl && npm install -g claudectl`

### Common Issues

- **Completions not loading**: Restart your terminal or run `exec $SHELL`
- **Permission errors**: Check that completion directories are writable
- **Mixed installations**: Remove old completion files before reinstalling
- **Dynamic task completion not working**: Ensure you're in a claudectl-initialized repository

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests with `cargo test`
5. Submit a pull request

## License

MIT - see [LICENSE](LICENSE) file for details.