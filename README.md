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

### Global Options

- `--debug`: Enable debug logging output

## Installation

```bash
npm install -g claudectl
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests with `cargo test`
5. Submit a pull request

## License

MIT - see [LICENSE](LICENSE) file for details.